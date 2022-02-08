table! {
    members (id) {
        id -> Int8,
        point -> Int4,
        last_daily -> Date,
    }
}

table! {
    point_record (id, date) {
        id -> Int8,
        point -> Int4,
        date -> Date,
    }
}

allow_tables_to_appear_in_same_query!(
    members,
    point_record,
);