use self::errors::Error;
use self::oidc::{
  authorize_url, init_oidc_client, AuthRequest, AuthRequestState,
  AuthTokenState, ClientState,
};
use self::props::client::ClientProps;
use self::query::LoginQuerySegments;
use ::dioxus::prelude::*;
use ::openidconnect::core::CoreClient;
use ::openidconnect::ClientId;

mod constants;
mod errors;
mod oidc;
mod props;
pub mod query;

#[derive(PartialEq, Props)]
pub struct LoginProps {
  pub query_params: LoginQuerySegments,
}

#[allow(non_snake_case)]
pub fn Login(cx: Scope<LoginProps>) -> Element {
  let use_state_auth_request_state: &UseState<AuthRequestState> =
    use_state(cx, || AuthRequestState {
      auth_request: None,
    });
  let use_state_auth_token_state: &UseState<AuthTokenState> =
    use_state(cx, || AuthTokenState {
      id_token: None,
      refresh_token: None,
    });
  let use_state_client_state: &UseState<ClientState> =
    use_state(cx, || ClientState {
      oidc_client: None,
    });
  let client_load_element: Element =
    make_client_load_element(cx, use_state_client_state);
  let auth_state_element: Element = make_auth_request_state_element(
    cx,
    use_state_auth_request_state,
    use_state_client_state,
  );
  let auth_token_state_element: Element = make_auth_token_element(
    cx,
    use_state_auth_request_state,
    use_state_auth_token_state,
  );
  render! {
  div {
    class: "app-login box",
  h1 {
  "Login"
  }
  h2 {
    "Props:"
    cx.props.query_params.placeholder.clone()
  }
  client_load_element
  auth_state_element
  auth_token_state_element
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

fn make_auth_request_element(
  cx: Scope<LoginProps>,
  auth_request: AuthRequest,
) -> Element {
  render! {
  p {
    "Auth Request URL:"
  br {
  }
  div {
  style: "white-space: pre-wrap",
  format!("{:#?}", auth_request)
  }
  }
  }
}

fn make_auth_request_state_element<'a>(
  cx: Scope<'a, LoginProps>,
  use_state_auth_request_state: &'a UseState<AuthRequestState>,
  use_state_client_state: &'a UseState<ClientState>,
) -> Element<'a> {
  if use_state_auth_request_state.auth_request.is_some() {
    return make_auth_request_element(
      cx,
      use_state_auth_request_state.auth_request.clone().unwrap(),
    );
  }
  if use_state_client_state.oidc_client.is_none() {
    return render! {
      p {
      "Waiting on the client to be loaded before making an Auth Request URL..."
      }
    };
  }
  let client_props: &ClientProps =
    use_state_client_state.oidc_client.as_ref().unwrap();
  let client: &CoreClient = &client_props.client;
  let auth_request: AuthRequest = authorize_url(client.clone());
  use_state_auth_request_state.set(AuthRequestState {
    auth_request: Some(auth_request.clone()),
  });
  render! {
    make_auth_request_element(cx, auth_request)
  }
}

fn make_auth_token_element<'a>(
  cx: Scope<'a, LoginProps>,
  use_state_auth_request_state: &'a UseState<AuthRequestState>,
  use_state_auth_token_state: &'a UseState<AuthTokenState>,
) -> Element<'a> {
  if use_state_auth_request_state.auth_request.is_none() {
    return render! {
      p {
      "Waiting on the Auth Request State to be loaded..."
      }
    };
  }
  if use_state_auth_token_state.id_token.is_none() {
    let auth_request_ref_option: Option<&AuthRequest> =
      use_state_auth_request_state.auth_request.as_ref();
    let auth_request_ref: &AuthRequest = auth_request_ref_option.unwrap();
    let authorize_url_str: &str = auth_request_ref.authorize_url.as_str();
    return render! {
      p {
      "Login:"
      a {
        href: authorize_url_str,
        target: "_blank",
        authorize_url_str
      }
      }
    };
  }
  let auth_token_state: &AuthTokenState = use_state_auth_token_state;
  render! {
  p {
  "You are logged in.  AuthTokenState:"
  br {
  }
  div {
  style: "white-space: pre-wrap",
  format!("{:#?}", auth_token_state)
  }
  }
  }
}

fn make_client_load_element<'a>(
  cx: Scope<'a, LoginProps>,
  use_state_client_state: &'a UseState<ClientState>,
) -> Element<'a> {
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
      p {
        style: "white-space: pre-wrap",
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
  cx: Scope<LoginProps>,
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
