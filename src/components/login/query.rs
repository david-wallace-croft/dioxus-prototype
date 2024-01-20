use ::dioxus_router::routable::FromQuery;
use ::serde::{Deserialize, Serialize};
use ::std::fmt::Display;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct LoginQuerySegments {
  pub placeholder: String,
}

impl Display for LoginQuerySegments {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "placeholder={}", self.placeholder)
  }
}

impl FromQuery for LoginQuerySegments {
  fn from_query(query: &str) -> Self {
    log::info!("query: {}", query);
    let mut placeholder_option = None;
    let pairs = ::form_urlencoded::parse(query.as_bytes());
    pairs.for_each(|(key, value)| {
      if key == "placeholder" {
        placeholder_option = Some(value.clone().into());
      }
    });
    Self {
      placeholder: placeholder_option.unwrap_or_default(),
    }
  }
}
