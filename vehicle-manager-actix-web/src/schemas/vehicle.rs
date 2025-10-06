use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct VehicleSchema {
    pub id: String,
    pub brand: String,
}
