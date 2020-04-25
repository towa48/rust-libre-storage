use rocket_contrib::json::Json;

#[derive(Serialize, Debug)]
pub struct ApiError {
    pub error: &'static str,
    pub error_description: &'static str,
}

impl ApiError {
    pub fn new(error: &'static str, error_description: &'static str) -> ApiError {
        ApiError { error: error, error_description: error_description }
    }
}

pub type ApiResult<T> = Result<Json<T>, Json<ApiError>>;
