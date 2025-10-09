use crate::models::vehicle::Vehicle;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
#[derive(Serialize, Deserialize, ToSchema)]
pub struct VehicleSchema {
    #[schema(example = "507f1f77bcf86cd799439011")]
    pub id: String,
    #[schema(example = "Audi")]
    pub brand: String,
    #[schema(example = "S3")]
    pub model: String,
    #[schema(example = 310)]
    pub ps: i32,
    #[schema(example = 10_000)]
    pub mileage_in_km: i32,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateVehicleSchema {
    #[schema(example = "Audi")]
    pub brand: String,
    #[schema(example = "S3")]
    pub model: String,
    #[schema(example = 310)]
    pub ps: i32,
    #[schema(example = 10_000)]
    pub mileage_in_km: i32,
}

impl From<Vehicle> for VehicleSchema {
    fn from(vehicle: Vehicle) -> Self {
        Self {
            id: vehicle._id.to_string(),
            brand: vehicle.brand,
            model: vehicle.model,
            ps: vehicle.ps,
            mileage_in_km: vehicle.mileage_in_km,
        }
    }
}
