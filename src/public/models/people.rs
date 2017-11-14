use super::Details;

#[derive(
    Serialize, Deserialize,
    Eq, PartialEq, Ord, PartialOrd,
    Debug, Clone
)]
pub struct People {
    pub name: String,
    pub profile_url: String,
    pub profile_picture: String,

    pub current_activity: Option<Details>,
    pub additional_information: Vec<Details>
}

impl People {

    pub fn new(name: &str, profile_url: &str, profile_picture: &str, current_activity: Option<Details>, additional_information: Vec<Details>) -> Self {
        People {
            name: String::from(name),
            profile_url: String::from(profile_url),
            profile_picture: String::from(profile_picture),
            
            current_activity: current_activity,
            additional_information: additional_information
        }
    }
}