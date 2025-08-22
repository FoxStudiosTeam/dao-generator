#[derive(Clone,Debug)]
#[fox_orm(table_name = "users")]
pub struct users {
    pub name: String,
    pub some: i128,
    pub code: String,
    pub created_by: String,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
}