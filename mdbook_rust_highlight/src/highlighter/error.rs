use thiserror::Error;

#[derive(Error, Debug)]
pub enum IdentificationError {
    #[error("Item already identified")]
    AlreadyIdentified,
    #[error("Item does not need identification")]
    NoIdentificationNeeded,
}
