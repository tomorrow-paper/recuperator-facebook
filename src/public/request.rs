use tomorrow_recuperator::Request;

pub struct FacebookPublicRequest {
    pub query: String
}

impl FacebookPublicRequest {

    pub fn new(query: &str, page: u8) -> Self {
        FacebookPublicRequest {
            query: format!("{}?page={}", query, page)
        }
    }
}

impl Request for FacebookPublicRequest {}