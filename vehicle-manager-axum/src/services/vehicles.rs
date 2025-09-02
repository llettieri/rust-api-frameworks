use crate::models::vehicle::Vehicle;

pub async fn get_vehicle(vehicle_id: String) -> Vehicle {
    Vehicle {
        id: vehicle_id,
        manufacturer: "Audio".to_string(),
        model: "S3".to_string(),
        year: 2025,
    }
}
