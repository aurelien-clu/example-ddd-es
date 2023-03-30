#[derive(thiserror::Error, Clone, Debug, PartialEq)]
pub enum Error {
    #[error("Unable to take off in curent state")]
    CannotTakeOff,
    #[error("Unable to land in current state")]
    CannotLand,
    #[error("Cannot register again, identification is immutable")]
    AlreadyRegistered,
}
