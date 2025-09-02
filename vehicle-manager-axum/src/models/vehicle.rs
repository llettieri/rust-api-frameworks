#[derive(Debug, serde::Serialize)]
pub struct Vehicle {
    pub(crate) id: String,
    pub(crate) manufacturer: String,
    pub(crate) model: String,
    pub(crate) year: u32,
}
