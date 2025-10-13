use crate::schemas::vehicle::{CreateVehicleSchema, VehicleSchema};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vehicle {
    pub _id: ObjectId,
    pub brand: String,
    pub model: String,
    pub ps: i32,
    pub mileage_in_km: i32,
}

/// Conversion from request schema to model entity.
impl From<CreateVehicleSchema> for Vehicle {
    fn from(vehicle: CreateVehicleSchema) -> Self {
        Self {
            _id: Default::default(),
            brand: vehicle.brand,
            model: vehicle.model,
            ps: vehicle.ps,
            mileage_in_km: vehicle.mileage_in_km,
        }
    }
}

/// Conversion from request schema to model entity.
impl From<VehicleSchema> for Vehicle {
    fn from(vehicle: VehicleSchema) -> Self {
        Self {
            _id: vehicle.id.parse().unwrap(),
            brand: vehicle.brand,
            model: vehicle.model,
            ps: vehicle.ps,
            mileage_in_km: vehicle.mileage_in_km,
        }
    }
}
