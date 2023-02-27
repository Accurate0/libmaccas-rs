mod api;
mod error;
pub mod types;

pub type ClientResult<T> = Result<T, ClientError>;

pub use api::ApiClient;
pub use error::ClientError;
pub use types::response::ClientResponse;
