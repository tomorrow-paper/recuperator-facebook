#[derive(
    Serialize, Deserialize,
    Eq, PartialEq, Ord, PartialOrd,
    Debug, Clone
)]
pub struct Details {
    pub label: String,
    pub url: String
}

impl Details {

    pub fn new(label: &str, url: &str) -> Self {
        Details {
            label: String::from(label),
            url: String::from(url)
        }
    }
}