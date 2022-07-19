mod api;
pub mod types;

pub(crate) type Error = anyhow::Error;
pub type ClientResult<T> = Result<T, Error>;

pub use api::ApiClient;
pub use types::response::ClientResponse;
