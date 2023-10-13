use dioxus::prelude::*;

static INVESTMENT_INTEREST: &str = "10.0";
static INVESTMENT_YEARS: &str = "50.0";
static RETIREMENT_INCOME: &str = "100000.0";
static RETIREMENT_INFLATION: &str = "1.0";
static RETIREMENT_INTEREST: &str = "10.0";
static RETIREMENT_TAX_RATE: &str = "10.0";

#[allow(non_snake_case)]
pub fn Retirement(cx: Scope) -> Element {
  let retirement_income: &UseState<String> =
    use_state(cx, || RETIREMENT_INCOME.to_string());
  let investment_years: &UseState<String> =
    use_state(cx, || INVESTMENT_YEARS.to_string());
  render! {
  div {
    class: "app-retirement box",
  h1 {
    class: "app-title",
    "Retirement"
  }
  div {
    class: "app-form",

  span {
    style: "white-space: pre-line",
    "Desired annual retirement income\n (present value, after taxes)"
  }
  input {
    size: "10",
    oninput: move |evt| retirement_income.set(evt.value.clone()),
    r#type: "text",
    value: "{retirement_income}",
  }
  span {
    style: "align-self: center",
    "dollars ($)"
  }

  span {
    style: "white-space: pre-line",
    "Years until retirement\n (usually at 67 years of age)"
  }
  input {
    size: "10",
    oninput: move |evt| investment_years.set(evt.value.clone()),
    r#type: "text",
    value: "{investment_years}",
  }
  span {
    style: "align-self: center",
    "years"
  }

  }
  p {
    "{retirement_income}"
  }
  p {
    "{investment_years}"
  }
  }
  }
}
