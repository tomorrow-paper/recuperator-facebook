use tomorrow_recuperator::Request;

pub struct FacebookProfileRequest {
    pub profile: String
}

impl FacebookProfileRequest {

    pub fn new(profile: &str) -> Self {
        FacebookProfileRequest {
            profile: String::from(profile)
        }
    }
}

impl Request for FacebookProfileRequest {}