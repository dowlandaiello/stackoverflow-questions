table! {
    roles (user_id) {
        id -> Unsigned<Bigint>,
        user_id -> Unsigned<Bigint>,
        administrator -> Nullable<Bool>,
        sponsor -> Nullable<Bool>,
        bot -> Nullable<Bool>,
    }
}

table! {
    users (id) {
        id -> Unsigned<Bigint>,
        username -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(
    roles,
    users,
);
