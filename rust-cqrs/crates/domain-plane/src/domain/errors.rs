#[derive(thiserror::Error, Clone, Debug, PartialEq)]
pub enum PlaneError {
    #[error("Unable to take off in curent state")]
    CannotTakeOff,
    #[error("Unable to land in current state")]
    CannotLand,
}
