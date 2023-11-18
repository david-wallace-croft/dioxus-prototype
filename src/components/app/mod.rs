use crate::route::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus_std::i18n::{use_init_i18n, Language};
use std::str::FromStr;

#[allow(non_snake_case)]
pub fn App(cx: Scope) -> Element {
  static EN_US: &str = r#"{
    "id": "en-US",
    "texts": {
      "messages": {
        "retirement_desired": "Desired annual retirement income\n(present value, after taxes)",
        "retirement_dollars": "dollars",
        "retirement_title": "Retirement"
      }
    }
  }"#;
  static ES_ES: &str = r#"{
    "id": "es-ES",
    "texts": {
      "messages": {
        "retirement_desired": "Ingresos anuales deseados para la jubilación\n(valor presente, después de impuestos)",
        "retirement_dollars": "dólares",
        "retirement_title": "Jubilación"
      }
    }
  }"#;
  use_init_i18n(
    cx,
    "en-US".parse().unwrap(), // selected_language
    "en-US".parse().unwrap(), // fallback_language
    || {
      vec![
        Language::from_str(EN_US).unwrap(),
        Language::from_str(ES_ES).unwrap(),
      ]
    },
  );
  render! {
    Router::<Route> { }
  }
}
