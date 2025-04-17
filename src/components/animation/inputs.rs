// TODO: Instead of a flag, maybe the original Event object
// TODO: Instead of events, maybe a specific type of request, like pause & play
#[derive(Default)]
pub struct Inputs {
  pub blur: bool,
  pub click: bool,
  pub drift: i8,
  pub focus: bool,
  pub pause: bool,
  pub play: bool,
  pub stop: bool,
}
