use crate::models::vehicle::Vehicle;
use futures::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{Document, doc, to_bson};
use mongodb::{Collection, Database};

#[derive(Debug, Clone)]
pub struct VehicleRepository {
    collection: Collection<Vehicle>,
}

impl VehicleRepository {
    pub fn new(database: Database) -> Self {
        Self {
            collection: database.collection("vehicle"),
        }
    }

    pub async fn save_vehicle(&self, vehicle: Vehicle) -> () {
        self.collection.insert_one(vehicle).await.unwrap();
    }

    pub async fn update_vehicle(&self, vehicle: Vehicle) -> Option<Vehicle> {
        let filter = doc! {"_id": vehicle._id};
        let update = doc! {"$set": to_bson(&vehicle).unwrap()};

        self.collection
            .update_one(filter.clone(), update)
            .await
            .unwrap_or_default();

        self.collection.find_one(filter).await.unwrap_or(None)
    }

    pub async fn paginate_vehicles(
        &self,
        filter: Option<Document>,
        page: u32,
        size: u32,
    ) -> (Vec<Vehicle>, u64) {
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
            Err(_) => return (vec![], 0),
        };
        let total = self.collection.count_documents(total_filter);

        (
            cursor.try_collect().await.unwrap_or(vec![]),
            total.await.unwrap_or(0),
        )
    }

    pub async fn get_vehicle_by_id(&self, vehicle_id: ObjectId) -> Option<Vehicle> {
        let result = self.collection.find_one(doc! {"_id": &vehicle_id}).await;

        result.unwrap_or_else(|_| None)
    }

    pub async fn delete_vehicle_by_id(&self, vehicle_id: ObjectId) -> () {
        self.collection
            .delete_one(doc! {"_id": &vehicle_id})
            .await
            .unwrap();
    }
}
