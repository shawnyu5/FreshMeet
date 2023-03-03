use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
/// User object response from EventBrite API
///
/// * `id`: id of the user
/// * `emails`: email of the user
/// * `name`: name of the user
/// * `first_name`: first name of the user
/// * `last_name`: last name of the user
/// * `is_public`: if the user profile is public
/// * `image_id`: id of the user's profile image. Can be null
pub struct User {
    id: String,
    emails: Vec<Email>,
    name: String,
    first_name: String,
    last_name: String,
    is_public: bool,
    image_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
/// The email object of the user
///
/// * `email`: email of the user
/// * `verified`: if the email is verified
/// * `primary`: if this is the primary email of the user
pub struct Email {
    email: String,
    verified: bool,
    primary: bool,
}

impl User {
    /// create a new User object
    pub fn new() -> User {
        return User {
            id: String::new(),
            emails: Vec::new(),
            name: String::new(),
            first_name: String::new(),
            last_name: String::new(),
            is_public: false,
            image_id: None,
        };
    }
    /// Use the EventBrite API to get the user's information
    pub async fn user_info(&self) -> Result<User, String> {
        let url = format!(
            "{}/users/me/?token={}",
            super::utils::api_address(),
            super::utils::api_key()
        );

        let response = reqwest::get(url).await.unwrap();
        match response.status() {
            reqwest::StatusCode::OK => match response.json::<User>().await {
                Ok(user) => {
                    return Ok(user);
                }
                Err(e) => {
                    return Err(e.to_string());
                }
            },
            reqwest::StatusCode::UNAUTHORIZED => {
                return Err("Unauthorized".to_string());
            }

            _ => {
                return Err("Invalid response".to_string());
            }
        }
    }
}
