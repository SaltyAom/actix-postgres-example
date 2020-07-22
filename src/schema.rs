table! {
    employees (id) {
        id -> Int4,
        firstname -> Varchar,
        lastname -> Varchar,
        department -> Varchar,
        salary -> Int4,
        age -> Int4,
    }
}

table! {
    user (username) {
        username -> Varchar,
        password -> Varchar,
    }
}

table! {
    users (username) {
        username -> Varchar,
        password -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    employees,
    user,
    users,
);
