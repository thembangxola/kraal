use yew::services::fetch::{Request, Method};
use serde::Serialize;

#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct RegisterFormData {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

pub fn register_user(data: &RegisterFormData) -> Request<Json<&RegisterFormData>> {
    Request::post("http://localhost:5500/api/register")
        .header("Content-Type", "application/json")
        .body(Json(data))
        .unwrap()
}
