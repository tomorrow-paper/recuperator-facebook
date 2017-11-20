#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use tomorrow_core::Result;
use tomorrow_http::Builder;
use tomorrow_http::raw::*;
use tomorrow_recuperator::Recuperator;

use select::document::Document;
use select::node::Node;
use select::predicate::{Comment, Class, Name};

use super::{FacebookProfileRequest, FacebookProfileResponse};
use super::models::*;

const API_URL: &'static str = "https://www.facebook.com";

pub struct FacebookProfileRecuperator<T> where T: Requester {
    requester: T
}

impl <T> FacebookProfileRecuperator<T> where T: Requester {
    
    pub fn new(requester: T) -> Self {
        FacebookProfileRecuperator {
            requester: requester
        }
    }

    fn extract_profile(&self, document: Document) -> Profile {
        Profile {
            name: String::from("Test")
        }
    }
}

impl <T> Recuperator<FacebookProfileRequest, FacebookProfileResponse> for FacebookProfileRecuperator<T> where T: Requester {

    fn compute(&self, request: FacebookProfileRequest) -> Result<FacebookProfileResponse> {
        let html = self.requester.request(&request.profile)?;
        let document = Document::from(html.as_ref());

        let profile = self.extract_profile(document);
        let response = FacebookProfileResponse::new(profile);

        Ok(response)
    }
}

impl Default for FacebookProfileRecuperator<Client> {

    fn default() -> Self {
        let client: Client = Builder::https(API_URL).lang("en-GB").into();
        FacebookProfileRecuperator::new(client)
    }
}