use com_croftsoft_core::math::finance_lib::PeriodicSavingsNeeded;
use dioxus::prelude::*;

static INVESTMENT_INTEREST: &str = "10.0";
static INVESTMENT_YEARS: &str = "50.0";
static RETIREMENT_INCOME: &str = "100000.0";
static RETIREMENT_INFLATION: &str = "1.0";
static RETIREMENT_INTEREST: &str = "10.0";
static RETIREMENT_TAX_RATE: &str = "10.0";

#[allow(non_snake_case)]
pub fn Retirement(cx: Scope) -> Element {
  let investment_interest: &UseState<String> =
    use_state(cx, || INVESTMENT_INTEREST.to_string());
  let investment_years: &UseState<String> =
    use_state(cx, || INVESTMENT_YEARS.to_string());
  let retirement_income: &UseState<String> =
    use_state(cx, || RETIREMENT_INCOME.to_string());
  let retirement_inflation: &UseState<String> =
    use_state(cx, || RETIREMENT_INFLATION.to_string());
  let retirement_interest: &UseState<String> =
    use_state(cx, || RETIREMENT_INTEREST.to_string());
  let retirement_tax_rate: &UseState<String> =
    use_state(cx, || RETIREMENT_TAX_RATE.to_string());
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
    "Desired annual retirement income\n(present value, after taxes)"
  }
  input {
    size: "10",
    oninput: move |evt| retirement_income.set(evt.value.clone()),
    r#type: "text",
    value: "{retirement_income}",
  }
  span {
    "dollars ($)"
  }

  span {
    style: "white-space: pre-line",
    "Years until retirement\n(usually at 67 years of age)"
  }
  input {
    size: "10",
    oninput: move |evt| investment_years.set(evt.value.clone()),
    r#type: "text",
    value: "{investment_years}",
  }
  span {
    "years"
  }

  span {
    style: "white-space: pre-line",
    "Annual investment growth rate\n(before retirement, tax-deferred)"
  }
  input {
    size: "10",
    oninput: move |evt| investment_interest.set(evt.value.clone()),
    r#type: "text",
    value: "{investment_interest}",
  }
  span {
    "percent (%)"
  }

  span {
    style: "white-space: pre-line",
    "Annual interest earned on savings\n(during retirement)"
  }
  input {
    size: "10",
    oninput: move |evt| retirement_interest.set(evt.value.clone()),
    r#type: "text",
    value: "{retirement_interest}",
  }
  span {
    "percent (%)"
  }

  span {
    style: "white-space: pre-line",
    "Tax rate on savings interest\n(during retirement)"
  }
  input {
    size: "10",
    oninput: move |evt| retirement_tax_rate.set(evt.value.clone()),
    r#type: "text",
    value: "{retirement_tax_rate}",
  }
  span {
    "percent (%)"
  }

  span {
    style: "white-space: pre-line",
    "Estimated annual inflation\n(before and during retirement)"
  }
  input {
    size: "10",
    oninput: move |evt| retirement_inflation.set(evt.value.clone()),
    r#type: "input",
    value: "{retirement_inflation}",
  }
  span {
    "percent (%)"
  }

  }
  }
  if calculate_required_annual_investment_from_state(
      investment_interest,
      investment_years,
      retirement_income,
      retirement_inflation,
      retirement_interest,
      retirement_tax_rate,
    ) < 0. {
    render! {
      p {
        style: "color: red",
        "The interest rate on retirement savings must exceed the annual \
        inflation rate."
      }
    }
  }
  p {
    "You would need to invest {calculate_required_annual_investment_from_state(
      investment_interest,
      investment_years,
      retirement_income,
      retirement_inflation,
      retirement_interest,
      retirement_tax_rate,
    )} each year."
  }
  p {
    "This calculator does not factor in social security income."
    br {}
    "Click "
    a {
      href: "https://www.bankrate.com/retirement/retirement-plan-calculator/",
      target: "_blank",
      "here"
    }
    " for a calculator that includes social security income."
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
  investment_interest: &UseState<String>,
  investment_years: &UseState<String>,
  retirement_income: &UseState<String>,
  retirement_inflation: &UseState<String>,
  retirement_interest: &UseState<String>,
  retirement_tax_rate: &UseState<String>,
) -> f64 {
  let desired_savings_interest_income: f64 = parse(retirement_income);
  let years_of_saving: f64 = parse(investment_years);
  let investment_interest_rate: f64 = parse(investment_interest) / 100.;
  let savings_interest: f64 = parse(retirement_interest) / 100.;
  let tax_rate: f64 = parse(retirement_tax_rate) / 100.;
  let inflation_rate: f64 = parse(retirement_inflation) / 100.;
  calculate_required_annual_investment(
    desired_savings_interest_income,
    years_of_saving,
    investment_interest_rate,
    savings_interest,
    tax_rate,
    inflation_rate,
  )
}

fn parse(state: &UseState<String>) -> f64 {
  state.get().parse().unwrap_or(0.)
}
