use std::io::{Error, ErrorKind};
use serde::(Deserialize, Serialize);

#[derive(Debug, Serialize, Deserialize)]

pub struct RegisterRequest {
    username: String,
    password: String,

}

pub fn register_user(req: RegisterRequest) -> Result<(), Box<dyn Error>> {

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn successful_registration() {
        let req = RegisterRequest {
            username: String::from("test_user"),
            password: String::from("secure_password"),
        };

        assert!(register_user(req).is_ok());
    }

    #[test]
    fn empty_username_fails() {
        let req = RegisterRequest {
            username: String::from(""),
            password: String::from("secure_password"),
        };

        let expected_error = Error::new(ErrorKind::InvalidInput, "Username cannot be empty");
        assert_eq!(format!("{:?}", register_user(req).err().unwrap()), format!("{:?}", expected_error));
    }
}