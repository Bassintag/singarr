use sqlx::prelude::FromRow;

#[derive(FromRow)]
pub struct IdRow {
    pub id: i64,
}
