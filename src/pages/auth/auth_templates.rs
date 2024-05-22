use askama::Template;

#[derive(Template)]
#[template(path = "components/auth/login.html")]
pub struct Login {}

#[derive(Template)]
#[template(path = "components/auth/loginSuccess.html")]
pub struct LoginSuccess {}

#[derive(Template)]
#[template(path = "pages/auth.html")]
pub struct AuthPage {}

#[derive(Template)]
#[template(path = "components/auth/register.html")]
pub struct Register {}
