use sqlx::postgres::PgRow;

pub trait FromPgRow {
    fn from_pg_row(row: PgRow) -> Self;
}
