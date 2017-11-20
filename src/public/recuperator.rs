use tomorrow_core::Result;
use tomorrow_http::Builder;
use tomorrow_http::raw::*;
use tomorrow_recuperator::Recuperator;

use select::document::Document;
use select::node::Node;
use select::predicate::{Comment, Class, Name};

use super::{FacebookPublicRequest, FacebookPublicResponse};
use super::models::*;

const API_URL: &'static str = "https://www.facebook.com/public";

pub struct FacebookPublicRecuperator<T> where T: Requester {
    requester: T
}

impl <T> FacebookPublicRecuperator<T> where T: Requester {
    
    pub fn new(requester: T) -> Self {
        FacebookPublicRecuperator {
            requester: requester
        }
    }

    fn extract_results(&self, document: Document) -> Vec<People> {
        self.extract_hidden_elements(document)
            .iter()
            .flat_map(|document| document.find(Class("_4p2o")))
            .map(|node| self.map_to_people(node))
            .collect()
    }

    fn extract_hidden_elements(&self, document: Document) -> Vec<Document> {
        document.find(Class("hidden_elem"))
            .flat_map(|hidden| hidden.find(Comment))
            .map(|comment| self.comment_to_document(comment))
            .collect()
    }

    fn comment_to_document(&self, comment: Node) -> Document {
        let uncommented = comment.html().replace("<!--", "").replace("-->", "");
        let html = format!("<!DOCTYPE html><html><body>{}</body></html>", uncommented);

        Document::from(html.as_ref())
    }

    fn map_to_people(&self, node: Node) -> People {
        let name = self.extract_name(&node);
        let profile_url = self.extract_profile_url(&node);
        let profile_picture = self.extract_profile_picture(&node);
        let current_activity = self.extract_current_activity(&node);
        let additional_information = self.extract_additional_information(&node);

        People::new(name.as_ref(), profile_url.as_ref(), profile_picture.as_ref(), current_activity, additional_information)
    }

    fn extract_name(&self, node: &Node) -> String {
        node.find(Class("_32mo"))
            .map(|div| div.text())
            .map(|name| String::from(name.trim()))
            .collect()
    }

    fn extract_profile_url(&self, node: &Node) -> String {
        node.find(Class("_gll"))
            .map(|div| div.find(Name("a")).next())
            .filter(Option::is_some)
            .map(Option::unwrap)
            .map(|a| a.attr("href"))
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect()
    }

    fn extract_profile_picture(&self, node: &Node) -> String {
        node.find(Class("_1glk"))
            .map(|img| img.attr("src"))
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect()
    }

    fn extract_current_activity(&self, node: &Node) -> Option<Details> {
        node.find(Class("_pac"))
            .map(|div| (div.text(), div.find(Name("a")).last()))
            .map(|(label, a)| (label, a.map(|a| a.attr("href")).map(|href| href.unwrap()).unwrap_or("")))
            .map(|(label, url)| (String::from(label.trim()), url))
            .map(|(label, url)| Details::new(label.as_ref(), url.as_ref()))
            .filter(|details| details.label.len() > 0)
            .next()
    }

    fn extract_additional_information(&self, node: &Node) -> Vec<Details> {
        node.find(Class("_52eh"))
            .map(|div| (div.text(), div.find(Name("a")).last()))
            .map(|(label, a)| (label, a.map(|a| a.attr("href")).map(|href| href.unwrap()).unwrap_or("")))
            .map(|(label, url)| (String::from(label.trim()), url))
            .map(|(label, url)| Details::new(label.as_ref(), url.as_ref()))
            .filter(|details| details.label.len() > 0)
            .collect::<Vec<Details>>()
    }
}

impl <T> Recuperator<FacebookPublicRequest, FacebookPublicResponse> for FacebookPublicRecuperator<T> where T: Requester {

    fn compute(&self, request: FacebookPublicRequest) -> Result<FacebookPublicResponse> {
        let html = self.requester.request(&request.query)?;
        let document = Document::from(html.as_ref());

        let results = self.extract_results(document);
        let response = FacebookPublicResponse::new(results);

        Ok(response)
    }
}

impl Default for FacebookPublicRecuperator<Client> {

    fn default() -> Self {
        let client: Client = Builder::https(API_URL).lang("en-GB").into();
        FacebookPublicRecuperator::new(client)
    }
}

#[cfg(test)]
mod tests {

    use tomorrow_recuperator::Recuperator;
    use tomorrow_http::raw::mock::MockClient;

    use ::public::*;
    use ::public::models::Details;

    #[test]
    fn public_recuperator_should_transform_document_into_people_vector() {
        let document = mock_document();
        let client = MockClient::with_content(document.as_ref());

        let recuperator = FacebookPublicRecuperator::new(client);
        let request = FacebookPublicRequest::new("Mock request", 1);

        let response = recuperator.compute(request);
        assert!(response.is_ok());
        
        let results = response.unwrap().results;
        assert_eq!(results.len(), 2);

        let mock_user_1 = &results[0];
        assert_eq!(mock_user_1.name, "Mock User 1");
        assert_eq!(mock_user_1.profile_url, "https://example.com/mock-user-1");
        assert_eq!(mock_user_1.profile_picture, "https://example.com/mock-user-1-profile-picture");
        
        assert!(mock_user_1.current_activity.is_some());
        assert_eq!(mock_user_1.current_activity, Some(Details::new("Mock User 1 Current Activity", "https://example.com/mock-user-1-current-activity")));
        
        assert_eq!(mock_user_1.additional_information.len(), 3);
        assert_eq!(mock_user_1.additional_information, vec![
            Details::new("From Native City", "https://example.com/native-city"),
            Details::new("Worked at My Workplace", "https://example.com/my-workplace"),
            Details::new("Studied at Some University", "https://example.com/some-university")
        ]);

        let mock_user_2 = &results[1];
        assert_eq!(mock_user_2.name, "Mock User 2");
        assert_eq!(mock_user_2.profile_url, "https://example.com/mock-user-2");
        assert_eq!(mock_user_2.profile_picture, "https://example.com/mock-user-2-profile-picture");
        
        assert!(mock_user_2.current_activity.is_none());
        assert_eq!(mock_user_2.additional_information.len(), 1);
        assert_eq!(mock_user_2.additional_information, vec![
            Details::new("From Native City", "https://example.com/native-city")
        ]);
    }

    fn mock_document() -> String {
        String::from(r#"<!DOCTYPE html>
        <html>
        <head>
            <title>Mock document</title>
        </head>
        <body>
            <div class="hidden_elem"><!--
                <div class="_4p2o">
                    <img alt="Mock User 1" class="_1glk img" src="https://example.com/mock-user-1-profile-picture">

                    <div class="_gll">
                        <div>
                            <a href="https://example.com/mock-user-1">
                                <div class="_32mo">
                                    Mock User 1
                                </div>
                            </a>
                        </div>
                    </div>

                    <div>
                        <div class="_glm">
                            <div class="_pac">
                                <a href="https://example.com/mock-user-1-current-activity">Mock User 1 Current Activity</a>
                            </div>
                        </div>
                        <div class="_glo">
                            <div>
                                <div class="_ajw">
                                    <div class="_52eh">
                                        From <a href="https://example.com/native-city">Native City</a>
                                    </div>
                                </div>
                                <div class="_ajw">
                                    <div class="_52eh">
                                        Worked at <a href="https://example.com/my-workplace">My Workplace</a>
                                    </div>
                                </div>
                                <div class="_ajw">
                                    <div class="_52eh">
                                        Studied at <a href="https://example.com/some-university">Some University</a>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="_4p2o">
                    <img alt="Mock User 2" class="_1glk img" src="https://example.com/mock-user-2-profile-picture">

                    <div class="_gll">
                        <div>
                            <a href="https://example.com/mock-user-2">
                                <div class="_32mo">
                                    Mock User 2
                                </div>
                            </a>
                        </div>
                    </div>

                    <div>
                        <div class="_glm">
                            <div class="_pac">
                            </div>
                        </div>
                        <div class="_glo">
                            <div>
                                <div class="_ajw">
                                    <div class="_52eh">
                                        From <a href="https://example.com/native-city">Native City</a>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            --></div>
        </body>
        </html>"#)
    }
}