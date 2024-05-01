use uuid::Uuid;


mod scylla;


pub trait Database {
    async fn add();
    async fn get(crawler_id: Uuid);
}