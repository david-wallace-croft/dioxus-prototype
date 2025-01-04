pub struct Card {
  pub answers: Vec<String>,
  pub correct_answer_index: usize,
}

impl Default for Card {
  fn default() -> Self {
    let answers: Vec<String> = vec![
      "0", "4", "12", "14", "42", "44", "48", "55", "84", "99",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();

    let correct_answer_index: usize = 6;

    Self {
      answers,
      correct_answer_index,
    }
  }
}
