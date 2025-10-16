use crate::models::repositories::BaseRepository;
use crate::models::vehicle::Vehicle;
use mongodb::Database;

/// Repository for managing vehicle entities in MongoDB.
/// The usage of the repository is restricted to the service layer.
pub type VehicleRepository = BaseRepository<Vehicle>;

impl VehicleRepository {
    /// Creates a new instance of the VehicleRepository.
    pub fn new(database: Database) -> Self {
        BaseRepository::init(database, String::from("vehicle"))
    }
}
