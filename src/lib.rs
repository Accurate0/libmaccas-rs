mod api;
pub mod types;

pub(crate) type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type ClientResult<T> = Result<T, Error>;

pub use api::ApiClient;
pub use types::ClientResponse;
