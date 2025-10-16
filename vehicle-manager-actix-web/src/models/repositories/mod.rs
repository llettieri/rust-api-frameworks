pub mod vehicle;

use crate::helpers::pagination::Page;
use crate::models::BaseDocument;
use futures::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, to_bson, Document};
use mongodb::{Collection, Database};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct BaseRepository<T: Send + Sync> {
    collection: Collection<T>,
}

impl<T> BaseRepository<T>
where
    T: BaseDocument + Serialize + DeserializeOwned + Unpin + Send + Sync  + Debug,
{
    pub fn init(database: Database, collection_name: String) -> Self {
        Self {
            collection: database.collection(&collection_name),
        }
    }

    pub async fn save(&self, item: T) -> () {
        self.collection.insert_one(item).await.unwrap();
    }

    pub async fn update(&self, item: T) -> Option<T> {
        let filter = doc! {"_id": item.id()};
        let update = doc! {"$set": to_bson(&item).unwrap()};

        self.collection
            .update_one(filter.clone(), update)
            .await
            .unwrap_or_default();

        self.collection.find_one(filter).await.unwrap_or(None)
    }

    pub async fn paginate(&self, filter: Option<Document>, page: u32, size: u32) -> Page<T> {
        let query_page = page.clone() - 1;
        let total_filter = filter.clone().unwrap_or_default();
        let skip: u64 = (query_page * size) as u64;

        let cursor = match self
            .collection
            .find(filter.unwrap_or_default())
            .skip(skip)
            .limit(size as i64)
            .await
        {
            Ok(cursor) => cursor,
            Err(_) => return Page::new(vec![], size, page, 0),
        };


        let total = self
            .collection
            .count_documents(total_filter)
            .await
            .unwrap_or(0);
        let items = cursor.try_collect().await.unwrap();

        println!("Total: {:?}", items);

        Page::new(items, size, page, total)
    }

    pub async fn find_by_id(&self, id: &ObjectId) -> Option<T> {
        self.collection
            .find_one(doc! {"_id": &id})
            .await
            .unwrap_or_else(|_| None)
    }

    pub async fn delete_by_id(&self, id: &ObjectId) -> () {
        self.collection.delete_one(doc! {"_id": &id}).await.unwrap();
    }
}
