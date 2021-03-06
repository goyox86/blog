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

table! {
    comments {
        id -> Integer,
        body -> VarChar,
        published -> Bool,
        user_id -> Integer,
        post_id -> Integer,
    }
}
