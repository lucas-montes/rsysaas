use crate::data::interfaces::db::Manager;
use crate::web::interface::View;
use axum::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow, Deserialize, Serialize, Default)]

pub struct Association {
    pub id: u32,
    pub table_related: String,
    pub row_id: u32,
}
#[async_trait]
impl Manager<'_> for Association {
    async fn table() -> String {
        "associations".to_string()
    }
}
#[async_trait]
impl View<'_> for Association {}
