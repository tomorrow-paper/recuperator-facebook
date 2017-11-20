#[derive(
    Serialize, Deserialize,
    Eq, PartialEq, Ord, PartialOrd,
    Debug, Clone
)]
pub struct Profile {
    pub name: String
}