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
        hashed_password -> Nullable<VarChar>,
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

table! {
    tokens {
        id -> Integer,
        value -> VarChar,
        user_id -> Integer,
    }
}
