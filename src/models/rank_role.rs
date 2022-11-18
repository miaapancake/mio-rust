use deadpool_postgres::tokio_postgres::Row;

pub struct RankRole {
    pub id: i64,
    pub guild_id: i64,
    pub role_id: i64,
    pub level: i32
}

impl From<Row> for RankRole {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            guild_id: row.get("guild_id"),
            level: row.get("level"),
            role_id: row.get("role_id")
        }
    }
}