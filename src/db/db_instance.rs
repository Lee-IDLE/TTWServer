use std::sync::Arc;
use tokio::sync::OnceCell;
use super::db_manager::Db_Manager;

static _db: OnceCell<Arc<Db_Manager>> = OnceCell::const_new();

pub async fn get_db_instance() -> Arc<Db_Manager> {
    _db.get_or_init(|| async {
        Arc::new(Db_Manager::new())
    }).await.clone()
}