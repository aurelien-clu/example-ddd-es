#[derive(thiserror::Error, Clone, Debug, PartialEq)]
pub enum Error {
    #[error("Unable to take off in curent state")]
    CannotTakeOff,
    #[error("Unable to land in current state")]
    CannotLand,
    #[error("Plane is already created, cannot its identification")]
    AlreadyCreated,
}
