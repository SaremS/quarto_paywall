use askama::Template;

#[derive(Template)]
#[template(path = "uber.html")]
pub struct UberTemplate {
    pub content: String,
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    pub error_message: String,
}

#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate {
    pub error_message: String
}

#[derive(Template)]
#[template(path = "register_success.html")]
pub struct RegisterSuccessTemplate {
    pub username: String,
}

#[derive(Template)]
#[template(path = "logout_success.html")]
pub struct LogoutSuccessTemplate {}

#[derive(Template)]
#[template(path = "login_success.html")]
pub struct LoginSuccessTemplate {
    pub username: String,
}

#[derive(Template)]
#[template(path = "user_dashboard.html")]
pub struct UserDashboardTemplate {
    pub username: String,
}

#[derive(Template)]
#[template(path = "delete_user.html")]
pub struct DeleteUserTemplate {}

#[derive(Template)]
#[template(path = "delete_user_confirmed.html")]
pub struct DeleteUserConfirmedTemplate {}
