use yew::{Component, Context, Html, Callback};
use yew_router::prelude::*;

#[derive(Debug)]
struct LoginData {
    email: String,
    password: String,
}

enum Msg {
    UpdateEmail(String),
    UdpatePassword(String),
    Submit,
    LoginSuccess(String),
    LoginError(String),
}

struct Login {
    link: Route<()>,
    on_submit: Callback<()>,
    data: LoginData,
    client: Client,
}

impl Component for Login {
    type Message = Msg;
    type Properties = Props;

    fn create (props; Self::Properties, link: Route<()>) ->Self {
        Self {
            link,
            on_submit::props.on_submit,
            data: LoginData {
                email: String::new(),
                password: String::new(),
            },
        }
    }

    fn update (&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateEmail(email)=>self.data.email = email,
            Msg::UdpatePassword(email)=>self.data.password = password,
            Msg::Submit => {
                let email = self.data.email.clone();
                let password = self.data.password.clone();
                self.client.post(format!("{}/login", backend_url))
                .json(&(&email, &password))
                .send()
                .expect("Failed to send login request")
                .text::<String>()
                .expect("Failed to read response text")
                .map(|response| Msg::LoginSuccess(response))
                .map_err(|err| Msg::LoginError(err.to_string()));
            }
            Msg::LoginSuccess(response) =>{
                log::info!("Login successful! Resposer: {}", response);
                
                let token = match serde_json::from_str::<LoginResponse>(&response) {
                    Ok(login_response) => login_response.token,
                    Err(err) => {
                        log::error!("Failed to parse login response: {}", err);
                        return
                    };
                    
                    use wasm_bindgen::prelude::*;
                    web_sys::window::Storage::local().unwrap().set_item("auth_token", &token).unwrap();

                    self.link.route("/");
                } 

            }
            Msg::LoginError(err) =>  {
                log::error!("Login failed: {}", err);
                self.error_message = Some(err); 
            }
            
        }
        true
    }


    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_email_change = Callback::from(|email| Msg::UpdateEmail(email));
        let on_password_change = Callback::from(|password| Msg::UpdatePassword(password));
        let on_submit = self.on_submit.clone();

        html! {
            <div class="login-form">
                <h1>Login</h1>
                <form onsubmit=ctx.link().callback(|_| Msg::Submit)>
                    <div class="form-group">
                        <label for="email">Email:</label>
                        <input type="email" id="email" value={self.data.email.clone()} onchange={on_email_change} />
                    </div>
                    <div class="form-group">
                        <label for="password">Password:</label>
                        <input type="password" id="password" value={self.data.password.clone()} onchange={on_password_change} />
                    </div>
                    <button type="submit">Login</button>
                </form>
                <p onclick={ctx.link().callback(|_| Msg::Submit)}>
                    { "Don't have an account? Register Here" }
                </p>
                <div class="error-message">
                    { self.error_message.as_ref().map(|err| html! { err }) }
                </div>
            </div>
        }
    }

}

#[derive(Properties, PartialEq)]
pub struct Props {
    on_submit: Callback<()>,
}