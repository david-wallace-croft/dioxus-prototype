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

  pub fn drift(
    &self,
    size: u8,
  ) -> Color {
    Color {
      blue: Color::drift_primary_color(self.blue, size),
      green: Color::drift_primary_color(self.green, size),
      red: Color::drift_primary_color(self.red, size),
    }
  }

  pub fn drift_primary_color(
    primary_color: u8,
    size: u8,
  ) -> u8 {
    let width: f64 = (size * 2 + 1) as f64;

    let delta: i8 = (random() * width) as i8 - size as i8;

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
