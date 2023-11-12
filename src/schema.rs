// @generated automatically by Diesel CLI.

diesel::table! {
    files (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        #[max_length = 255]
        hash -> Nullable<Varchar>,
        data -> Nullable<Mediumblob>,
    }
}
