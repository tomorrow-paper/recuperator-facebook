use tomorrow_recuperator::Response;

use super::models::People;

pub struct FacebookPublicResponse {
    pub results: Vec<People>
}

impl FacebookPublicResponse {

    pub fn new(results: Vec<People>) -> Self {
        FacebookPublicResponse {
            results: results
        }
    }
}

impl Response for FacebookPublicResponse {}