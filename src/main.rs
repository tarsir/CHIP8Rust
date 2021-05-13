mod system;
mod utils;
use utils::Result;

fn main() -> Result<()> {
    println!("{:?}", system::display::render()?);
    Ok(())
}
