use ::web_sys::js_sys::Math::random;

#[derive(Clone, Copy)]
pub struct Color {
  pub blue: u8,
  pub green: u8,
  pub red: u8,
}

impl Color {
  pub fn as_fill_style_string(&self) -> String {
    let &Color {
      blue,
      green,
      red,
    } = self;

    format!("rgb({red}, {green}, {blue})")
  }

  pub fn drift(&self) -> Color {
    Color {
      blue: Color::drift_primary_color(self.blue),
      green: Color::drift_primary_color(self.green),
      red: Color::drift_primary_color(self.red),
    }
  }

  pub fn drift_primary_color(primary_color: u8) -> u8 {
    let delta: i8 = (random() * 13.) as i8 - 6;

    primary_color.saturating_add_signed(delta)
  }

  pub fn generate_random_color() -> Color {
    let red: u8 = (random() * 256.) as u8;

    let green: u8 = (random() * 256.) as u8;

    let blue: u8 = (random() * 256.) as u8;

    Color {
      blue,
      green,
      red,
    }
  }
}
