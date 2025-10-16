pub mod repositories;
pub mod vehicle;

use mongodb::bson::oid::ObjectId;

pub trait BaseDocument {
    fn id(&self) -> ObjectId;
    // fn created_at(&self) -> chrono::DateTime<Utc>;
    // fn updated_at(&self) -> chrono::DateTime<Utc>;
}
