use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct VehicleSchema {
    pub id: String,
    pub brand: String,
}
