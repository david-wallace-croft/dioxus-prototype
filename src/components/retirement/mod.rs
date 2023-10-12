use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Retirement(cx: Scope) -> Element {
  let desired_income: &UseState<String> =
    use_state(cx, || "100000.0".to_string());
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
      "Desired annual retirement income (present value, after taxes)"
  }
  input {
    value: "{desired_income}",
    oninput: move |evt| desired_income.set(evt.value.clone()),
  }
  }
  span {
    "{desired_income}"
  }
  }
  }
}
