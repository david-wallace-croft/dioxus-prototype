use self::oidc::init_oidc_client;
use dioxus::prelude::*;

mod constants;
mod errors;
mod oidc;
mod props;

#[allow(non_snake_case)]
pub fn Login(cx: Scope) -> Element {
  let init_client_future =
    use_future(cx, (), |_| async move { init_oidc_client().await });
  render! {
  div {
    class: "app-login box",
  h1 {
  "Login"
  }
  p {
  "Click on the following to log into the application:"
  br { }
  a {
    href: "https://73yqz7i9oi-69cbfasnr72e657dj72qu0cprp\
      .auth.us-east-1.amazoncognito.com/\
      login?client_id=69cbfasnr72e657dj72qu0cprp\
      &response_type=code\
      &scope=openid\
      &redirect_uri=http%3A%2F%2Flocalhost%3A8080",
  "https://73yqz7i9oi-69cbfasnr72e657dj72qu0cprp\
    .auth.us-east-1.amazoncognito.com/login\
    ?client_id=69cbfasnr72e657dj72qu0cprp\
    &response_type=code\
    &scope=openid\
    &redirect_uri=http%3A%2F%2Flocalhost%3A8080"
  }
  }
  }
  }
}
