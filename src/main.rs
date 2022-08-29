use color_eyre::Result;
use tracing::info;

use crate::wm::Wm;

pub mod env;
pub mod events;
pub mod wm;

fn main() -> Result<()> {
    env::setup()?;

    info!("Initializing zwm");

    let mut wm = Wm::new(Some(":1"))?;
    wm.setup()?;
    wm.run()?;

    info!("Shutting down zwm");

    Ok(())
}
