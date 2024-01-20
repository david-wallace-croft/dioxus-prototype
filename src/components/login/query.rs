use ::dioxus_router::routable::FromQuery;
use ::serde::{Deserialize, Serialize};
use ::std::fmt::Display;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct LoginQuerySegments {
  pub placeholder_option: Option<String>,
}

impl Display for LoginQuerySegments {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    write!(
      f,
      // "placeholder={}",
      // self.placeholder_option.as_ref().unwrap_or(&String::new())
      ""
    )
  }
}

impl FromQuery for LoginQuerySegments {
  fn from_query(query: &str) -> Self {
    let mut placeholder_option = None;
    let pairs = ::form_urlencoded::parse(query.as_bytes());
    pairs.for_each(|(key, value)| {
      if key == "placeholder" {
        placeholder_option = Some(value.clone().into());
      }
    });
    Self {
      placeholder_option,
    }
  }
}
