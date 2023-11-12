use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::files)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct File {
    pub id: u64,
    pub name: Option<String>,
    pub hash: Option<String>,
    pub data: Option<Vec<u8>>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::files)]
pub struct NewFile<'a> {
    pub name: &'a str,
    pub hash: &'a str,
    pub data: &'a[u8],
}