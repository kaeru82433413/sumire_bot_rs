use diesel::Queryable;
use chrono::NaiveDate;
use crate::schema::{members, point_record};

#[derive(Queryable, Identifiable, AsChangeset, Debug)]
pub struct Member {
    pub id: i64,
    pub point: i32,
    pub last_daily: NaiveDate,
}


#[derive(Queryable, Debug)]
pub struct PointRecord {
    pub id: i64,
    pub point: i32,
    pub display_name: String,
    pub date: NaiveDate,
}

#[derive(Insertable, Debug)]
#[table_name = "point_record"]
pub struct PointRecordForm {
    pub id: i64,
    pub point: i32,
    pub date: NaiveDate,
}