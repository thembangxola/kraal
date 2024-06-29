use std::io::{Error, ErrorKind};
use serde::(Deserialize, Serialize);

#[derive(Debug, Serialize, Deserialize)]

pub struct RegisterRequest {
    username: String,
    password: String,

}

pub fn register_user(req: RegisterRequest) -> Result<(), Box<dyn Error>> {
    unsafe {
        let mut users = USERS.get_or_insert(HashMap::new());

        // validate username
        if users.contains_key(&req.username) {
            return Err(Box::new(Error::new(ErrorKind::AlreadyExists, "Username already exists")));
        }

        if req.username.is_empty() {
            return Err(Box::new(Error::new(ErrorKind::InvalidInput, "Username cannot be empty")));
        }

        // validate password
        if req.password.len() < 8 {
            return Err(Box::new(Error::new(ErrorKind::InvalidInput, "Password must be at least 8 characters")));
        }

        // store username and hashed password
        users.insert(req.username.clone(), req.password.clone());
        println!("User registered: {}", req.username);

        Ok(())
    }

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
    fn username_already_exists() {
        let req1 = RegisterRequest {
            username: String::from("existing_user"),
            password: String::from("password1"),
        };

        assert!(register_user(req1).is_ok());

        let req2 = RegisterRequest {
            username: String::from("existing_user"),
            password: String::from("password2"),
        };

        let expected_error = Error::new(ErrorKind::AlreadyExists, "Username already exists");
        assert_eq!(format!("{:?}", register_user(req2).err().unwrap()), format!("{:?}", expected_error));
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

    #[test]
    fn short_password_fails() {
        let req = RegisterRequest {
            username: String::from("test_user"),
            password: String::from("short"),
        };

        let expected_error = Error::new(ErrorKind::InvalidInput, "Password must be at least 8 characters");
        assert_eq!(format!("{:?}", register_user(req).err().unwrap()), format!("{:?}", expected_error));
    }
}