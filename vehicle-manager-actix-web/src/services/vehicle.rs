use crate::helpers::pagination::Page;
use crate::models::repositories::vehicle::VehicleRepository;
use crate::models::vehicle::Vehicle;
use crate::schemas::vehicle::{CreateVehicleSchema, VehicleSchema};
use mongodb::bson::oid;
use mongodb::bson::oid::ObjectId;
use mongodb::Database;

#[derive(Debug, Clone)]
pub struct VehicleService {
    vehicle_repository: VehicleRepository,
}

impl VehicleService {
    pub fn new(database: Database) -> Self {
        Self {
            vehicle_repository: VehicleRepository::new(database),
        }
    }

    pub async fn get_vehicles(&self, page: u32, size: u32) -> Page<Vehicle> {
        self.vehicle_repository.paginate(None, page, size).await
    }

    pub async fn create_vehicle(&self, vehicle: CreateVehicleSchema) {
        self.vehicle_repository.save(vehicle.into()).await
    }

    pub async fn get_vehicle_by_id(
        &self,
        vehicle_id: &String,
    ) -> Result<Option<Vehicle>, oid::Error> {
        let vehicle_id = ObjectId::parse_str(vehicle_id)?;

        Ok(self.vehicle_repository.find_by_id(&vehicle_id).await)
    }

    pub async fn update_vehicle(&self, vehicle: VehicleSchema) -> Option<Vehicle> {
        self.vehicle_repository.update(vehicle.into()).await
    }

    pub async fn delete_vehicle(&self, vehicle_id: &String) -> Result<(), oid::Error> {
        let vehicle_id = ObjectId::parse_str(vehicle_id)?;

        Ok(self.vehicle_repository.delete_by_id(&vehicle_id).await)
    }
}
