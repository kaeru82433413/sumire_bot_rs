use std::env;
use std::collections::HashSet;
use serenity::prelude::*;
use serenity::model::guild::Member;
use serenity::client::Client;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use crate::schema::members::dsl as members_table;
use crate::model::Member as MemberModel;

type PgConnectionManager = ConnectionManager<PgConnection>;
type PooledPgConnection = PooledConnection<PgConnectionManager>;
type PgConnectionPool = Pool<PgConnectionManager>;


struct PoolKey;
impl TypeMapKey for PoolKey {
    type Value = PgConnectionPool;
}


pub async fn insert_pool(client: &Client) {
    let mut data = client.data.write().await;
    let url = env::var("DATABASE_URL").unwrap();
    let manager = PgConnectionManager::new(url);
    let pool = PgConnectionPool::builder().max_size(4).build(manager).unwrap();
    data.insert::<PoolKey>(pool);
}

pub async fn get_connection(ctx: &Context) -> PooledPgConnection {
    let data = ctx.data.read().await;
    let pool = data.get::<PoolKey>().unwrap();
    pool.get().unwrap()
}



pub fn get_members_data(conn: &PgConnection, members: &[Member]) -> QueryResult<Vec<MemberModel>> {
    let mut ids: HashSet<_> = members.iter().map(|m| m.user.id.0 as i64).collect();

    let mut res = members_table::members.filter(members_table::id.eq_any(&ids))
        .load::<MemberModel>(conn)?;
    
    for MemberModel { id, .. } in res.iter() {
        ids.remove(id);
    }
    for id in ids {
        res.push(get_member_data(conn, id)?);
    }
    
    Ok(res)
}

pub fn get_member_data(conn: &PgConnection, member_id: i64) -> QueryResult<MemberModel> {
    let res: Option<MemberModel> = members_table::members.find(member_id)
        .get_result(conn).optional()?;
    Ok(match res {
        Some(res) => res,
        None => {
            diesel::insert_into(members_table::members).values(members_table::id.eq(member_id))
            .get_result(conn)?
        },
    })
}