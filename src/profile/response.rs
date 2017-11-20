use tomorrow_recuperator::Response;

use super::models::Profile;

#[derive(Debug)]
pub struct FacebookProfileResponse {
    pub profile: Profile
}

impl FacebookProfileResponse {

    pub fn new(profile: Profile) -> Self {
        FacebookProfileResponse {
            profile: profile
        }
    }
}

impl Response for FacebookProfileResponse {}