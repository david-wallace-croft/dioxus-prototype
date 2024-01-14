use self::oidc::{init_oidc_client, ClientState};
use self::props::client::ClientProps;
use dioxus::prelude::*;
use errors::Error;
use openidconnect::core::CoreClient;
use openidconnect::ClientId;

mod constants;
mod errors;
mod oidc;
mod props;

#[allow(non_snake_case)]
pub fn Login(cx: Scope) -> Element {
  let client_load_element: Element = make_client_load_element(cx);
  render! {
  div {
    class: "app-login box",
  h1 {
  "Login"
  }
  client_load_element
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

fn make_client_load_element(cx: Scope) -> Element {
  let use_state_client_state: &UseState<ClientState> =
    use_state(cx, || ClientState {
      oidc_client: None,
    });
  if use_state_client_state.oidc_client.is_some() {
    return make_client_state_element(
      cx,
      use_state_client_state.oidc_client.clone().unwrap(),
    );
  }
  let init_client_future: &UseFuture<Result<(ClientId, CoreClient), Error>> =
    use_future(cx, (), |_| async move { init_oidc_client().await });
  let option: Option<&Result<(ClientId, CoreClient), Error>> =
    init_client_future.value();
  if option.is_none() {
    return render! {
      p {
      "Loading client; please wait..."
      }
    };
  }
  let result: &Result<(ClientId, CoreClient), Error> = option.unwrap();
  let result_ref: Result<&(ClientId, CoreClient), &Error> = result.as_ref();
  if result.is_err() {
    let error: &Error = result_ref.unwrap_err();
    return render! {
      p {
      "Error loading client:"
      br { }
      pre {
      format!("{:#?}", error)
      }
      }
    };
  }
  let result_value: &(ClientId, CoreClient) = result_ref.unwrap();
  let client_id: &ClientId = &result_value.0;
  let client: &CoreClient = &result_value.1;
  let client_props = ClientProps::new(client_id.clone(), client.clone());
  let client_props_option: Option<ClientProps> = Some(client_props);
  let client_state = ClientState {
    oidc_client: client_props_option,
  };
  use_state_client_state.set(client_state);
  return render! {
    p {
    format!("Client loaded with identifier {:?}", client_id)
    }
    p {
    "Client:"
    br { }
    p {
      style: "white-space: pre-wrap",
    format!("{:#?}", client)
    }
    }
  };
}

fn make_client_state_element(
  cx: Scope,
  client_props: ClientProps,
) -> Element {
  return render! {
    p {
    "Client alread loaded:"
    br { }
    p {
      style: "white-space: pre-wrap",
    format!("{:#?}", client_props)
    }
    }
  };
}
