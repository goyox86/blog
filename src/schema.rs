table! {
    posts {
        id -> Integer,
        title -> VarChar,
        body -> VarChar,
        published -> Bool,
        user_id -> Nullable<Integer>,
    }
}

table! {
    users {
        id -> Integer,
        username -> VarChar,
        name -> VarChar,
        email -> VarChar,
    }
}
