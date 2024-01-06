use crate::components::login::types::AliasClient;

use self::oidc::init_oidc_client;
use dioxus::prelude::*;
use openidconnect::ClientId;

mod constants;
mod errors;
mod oidc;
mod props;
mod types;

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

  match init_client_future.value() {
    Some(client_props) => match client_props {
      Ok(result_value) => {
        let client_id: &ClientId = &result_value.0;
        let client: &AliasClient = &result_value.1;
        rsx! {
          pre {
            format!("{:#?}", client_id)
          }
          pre {
            format!("{:#?}", client)
          }
        }
      }
      Err(e) => {
        rsx! {
          pre {
            format!("{:#?}", e)
          }
        }
      }
    },
    None => {
      rsx! {
        p {
        "Loading client, please wait"
        }
      }
    }
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
