use serde::Serialize;

#[derive(Serialize)]
pub struct QueryResponse<'a, T: Serialize + 'a> {
    pub success: bool,
    pub data: Option<&'a T>,
    pub error: Option<String>,
    pub message: Option<String>,
}

pub fn make_query_response<'a, T: Serialize + 'a>(
    success: bool,
    data: Option<&'a T>,
    error: Option<&'a str>,
    message: Option<&'a str>,
) -> QueryResponse<'a, T> {
    QueryResponse {
        success,
        data,
        error: error.map(String::from),
        message: message.map(String::from),
    }
}