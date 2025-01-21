use self::reset_button::ResetButton;
use super::translator::Translator;
use crate::components::language_select::LanguageSelector;
use ::dioxus::prelude::*;
use com_croftsoft_core::math::finance_lib::PeriodicSavingsNeeded;
use com_croftsoft_lib_string::to_dollars;

mod reset_button;

static INVESTMENT_INTEREST: &str = "10.0";
static INVESTMENT_YEARS: &str = "50.0";
static RETIREMENT_INCOME: &str = "100,000.0";
static RETIREMENT_INFLATION: &str = "1.0";
static RETIREMENT_INTEREST: &str = "10.0";
static RETIREMENT_TAX_RATE: &str = "10.0";

#[allow(non_snake_case)]
#[component]
pub fn Retirement() -> Element {
  static CSS: Asset = asset!("/assets/app-retirement.css");

  let mut investment_interest: Signal<String> =
    use_signal(|| INVESTMENT_INTEREST.to_string());

  let mut investment_years: Signal<String> =
    use_signal(|| INVESTMENT_YEARS.to_string());

  let mut retirement_income: Signal<String> =
    use_signal(|| RETIREMENT_INCOME.to_string());

  let mut retirement_inflation: Signal<String> =
    use_signal(|| RETIREMENT_INFLATION.to_string());

  let mut retirement_interest: Signal<String> =
    use_signal(|| RETIREMENT_INTEREST.to_string());

  let mut retirement_tax_rate: Signal<String> =
    use_signal(|| RETIREMENT_TAX_RATE.to_string());

  rsx! {
  document::Stylesheet {
    href: CSS
  }
  div {
    class: "app-retirement box",
  div {
    margin_bottom: "1rem",
    text_align: "right",
  LanguageSelector { }
  }
  h1 {
    class: "app-title",
    Translator {
      en: "Retirement",
      es: "Jubilación",
    }
  }
  div {
    class: "app-form",

  span {
    style: "white-space: pre-line",
    Translator {
      en: "Desired annual retirement income\n(present value, after taxes)",
      es:
        r#"Ingresos anuales deseados para la jubilación
        (valor presente, después de impuestos)"#,
    }
  }
  input {
    size: "10",
    oninput: move |event| on_input(event, &mut retirement_income),
    r#type: "text",
    value: "{retirement_income}",
  }
  span {
    Translator {
      en: "dollars",
      es: "dólares",
    }
    " ($)"
  }

  span {
    style: "white-space: pre-line",
    Translator {
      en: "Years until retirement\n(usually at 67 years of age)",
      es: "Años hasta la jubilación\n(normalmente a los 67 años)",
    }
  }
  input {
    size: "10",
    oninput: move |event| on_input(event, &mut investment_years),
    r#type: "text",
    value: "{investment_years}",
  }
  span {
    Translator {
      en: "years",
      es: "años",
    }
  }

  span {
    style: "white-space: pre-line",
    Translator {
      en:
        r#"Annual investment growth rate
        (before retirement, tax-deferred)"#,
      es:
        r#"Tasa de crecimiento anual de la inversión
        (antes de la jubilación, con impuestos diferidos)"#,
    }
  }
  input {
    size: "10",
    oninput: move |event| on_input(event, &mut investment_interest),
    r#type: "text",
    value: "{investment_interest}",
  }
  span {
    Translator {
      en: "percent",
      es: "por ciento",
    }
    " (%)"
  }

  span {
    style: "white-space: pre-line",
    Translator {
      en: "Annual interest earned on savings\n(during retirement)",
      es: "Interés anual ganado sobre ahorros\n(durante la jubilación)",
    }
  }
  input {
    size: "10",
    oninput: move |event| on_input(event, &mut retirement_interest),
    r#type: "text",
    value: "{retirement_interest}",
  }
  span {
    Translator {
      en: "percent",
      es: "por ciento",
    }
    " (%)"
  }

  span {
    style: "white-space: pre-line",
    Translator {
      en:
        r#"Tax rate on savings interest
        (during retirement)"#,
      es:
        r#"Tasa impositiva sobre los intereses de ahorro
        (durante la jubilación)"#,
    }
  }
  input {
    size: "10",
    oninput: move |event| on_input(event, &mut retirement_tax_rate),
    r#type: "text",
    value: "{retirement_tax_rate}",
  }
  span {
    Translator {
      en: "percent",
      es: "por ciento",
    }
    " (%)"
  }

  span {
    style: "white-space: pre-line",
    Translator {
      en: "Estimated annual inflation\n(before and during retirement)",
      es: "Inflación anual estimada\n(antes y durante la jubilación)",
    }
  }
  input {
    size: "10",
    oninput: move |event| on_input(event, &mut retirement_inflation),
    r#type: "input",
    value: "{retirement_inflation}",
  }
  span {
    Translator {
      en: "percent",
      es: "por ciento",
    }
    " (%)"
  }
  }

  if input_is_empty(
    &investment_interest,
    &investment_years,
    &retirement_income,
    &retirement_inflation,
    &retirement_interest,
    &retirement_tax_rate,
  ) {
    p {
      style: "color: #F44; text-align: center; white-space: pre-line",
      Translator {
        en:
          r#"One or more of the inputs is invalid.
          Click Reset for the default values."#,
        es:
          r#"Una o más de las entradas no son válidas.
          Haga clic en Restablecer para ver los valores predeterminados."#,
      }
    }
  } else if calculate_required_annual_investment_from_state(
    &investment_interest,
    &investment_years,
    &retirement_income,
    &retirement_inflation,
    &retirement_interest,
    &retirement_tax_rate,
  ) < 0. {
    p {
      style: "color: #F44; text-align: center; white-space: pre-line",
      Translator {
        en:
          r#"The interest rate on retirement savings
          must exceed the annual inflation rate."#,
        es:
          r#"La tasa de interés de los ahorros para la jubilación
          debe exceder la tasa de inflación anual."#,
      }
    }
  } else {
    p {
      style: "text-align: center",
      Translator {
        en: "You would need to invest this amount each year:",
        es: "Necesitaría invertir esta cantidad cada año:",
      }
      br {}
      span {
      "{
        to_dollars(calculate_required_annual_investment_from_state(
        &investment_interest,
        &investment_years,
        &retirement_income,
        &retirement_inflation,
        &retirement_interest,
        &retirement_tax_rate,
      ))
      }"
      }
    }
  }

  div {
    style: "text-align:center",
  ResetButton {
    // TODO: Disable when the inputs are pristine
    disabled: false,
    on_click: move |_event| on_click_reset_button(
      &mut investment_interest,
      &mut investment_years,
      &mut retirement_income,
      &mut retirement_inflation,
      &mut retirement_interest,
      &mut retirement_tax_rate,
    ),
  }
  }

  p {
    style: "text-align: center",
    Translator {
      en:
        "This calculator does not factor in social security income.",
      es:
        r#"Esta calculadora no tiene en cuenta los ingresos de la seguridad
        social."#,
    }
    br {}
    Translator {
      en: "Click ",
      es: "Haga clic ",
    }
    a {
      href: "https://www.bankrate.com/retirement/retirement-plan-calculator/",
      target: "_blank",
      Translator {
        en: "here",
        es: "aquí",
      }
    }
    Translator {
      en:
        " for a calculator that includes social security income.",
      es:
        r#" para obtener una calculadora que incluye los ingresos de la
        seguridad social."#,
    }
  }
  }
  }
}

fn calculate_required_annual_investment(
  desired_savings_interest_income: f64,
  years_of_saving: f64,
  investment_interest_rate: f64,
  savings_interest_rate: f64,
  tax_rate: f64,
  inflation_rate: f64,
) -> f64 {
  let savings: f64 = desired_savings_interest_income * (1.0 + inflation_rate)
    / (savings_interest_rate * (1.0 - tax_rate) - inflation_rate);

  if years_of_saving == 0.0 {
    return savings;
  }

  let future_value_savings =
    savings * (1.0 + inflation_rate).powf(years_of_saving);

  PeriodicSavingsNeeded {
    future_value: future_value_savings,
    interest_rate: investment_interest_rate,
    time_periods: years_of_saving,
  }
  .calculate()
}

fn calculate_required_annual_investment_from_state(
  investment_interest: &Signal<String>,
  investment_years: &Signal<String>,
  retirement_income: &Signal<String>,
  retirement_inflation: &Signal<String>,
  retirement_interest: &Signal<String>,
  retirement_tax_rate: &Signal<String>,
) -> f64 {
  let desired_savings_interest_income: f64 =
    parse_signal_or_zero(retirement_income);

  let years_of_saving: f64 = parse_signal_or_zero(investment_years);

  let investment_interest_rate: f64 =
    parse_signal_or_zero(investment_interest) / 100.;

  let savings_interest_rate: f64 =
    parse_signal_or_zero(retirement_interest) / 100.;

  let tax_rate: f64 = parse_signal_or_zero(retirement_tax_rate) / 100.;

  let inflation_rate: f64 = parse_signal_or_zero(retirement_inflation) / 100.;

  calculate_required_annual_investment(
    desired_savings_interest_income,
    years_of_saving,
    investment_interest_rate,
    savings_interest_rate,
    tax_rate,
    inflation_rate,
  )
}

fn input_is_empty(
  investment_interest: &Signal<String>,
  investment_years: &Signal<String>,
  retirement_income: &Signal<String>,
  retirement_inflation: &Signal<String>,
  retirement_interest: &Signal<String>,
  retirement_tax_rate: &Signal<String>,
) -> bool {
  parse_signal(investment_interest).is_none()
    || parse_signal(investment_years).is_none()
    || parse_signal(retirement_income).is_none()
    || parse_signal(retirement_inflation).is_none()
    || parse_signal(retirement_interest).is_none()
    || parse_signal(retirement_tax_rate).is_none()
}

fn on_click_reset_button(
  investment_interest: &mut Signal<String>,
  investment_years: &mut Signal<String>,
  retirement_income: &mut Signal<String>,
  retirement_inflation: &mut Signal<String>,
  retirement_interest: &mut Signal<String>,
  retirement_tax_rate: &mut Signal<String>,
) {
  investment_interest.set(INVESTMENT_INTEREST.to_owned());

  // TODO: use set() instead of write()?

  *investment_years.write() = INVESTMENT_YEARS.to_owned();

  *retirement_income.write() = RETIREMENT_INCOME.to_owned();

  *retirement_inflation.write() = RETIREMENT_INFLATION.to_owned();

  *retirement_interest.write() = RETIREMENT_INTEREST.to_owned();

  *retirement_tax_rate.write() = RETIREMENT_TAX_RATE.to_owned();
}

fn on_input(
  event: Event<FormData>,
  state: &mut Signal<String>,
) {
  let value: String = event.data.value();

  if value.is_empty() || parse_string_slice(&value).is_some() {
    state.set(value);
  } else {
    let old_value: String = state.read().clone();

    // TODO: Does this work?

    state.set(old_value);
  }
}

fn parse_signal(signal: &Signal<String>) -> Option<f64> {
  let value = signal.read();

  parse_string_slice(&value)
}

fn parse_signal_or_zero(state: &Signal<String>) -> f64 {
  parse_signal(state).unwrap_or(0.)
}

fn parse_string_slice(value: &str) -> Option<f64> {
  value.replace(',', "").parse().ok()
}
