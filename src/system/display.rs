use crate::utils::Result;

static DISPLAY: [[bool; 64]; 32] = [[true; 64]; 32];

pub fn render() -> Result<()> {
  for col in &DISPLAY {
    println!("{}", &col.iter().map(|pixel| {
      if *pixel {
        block_char()
      } else {
        ' '
      }
    })
    .collect::<String>());
  }
  Ok(())
}

fn block_char() -> char {
  '\u{2588}'
}
