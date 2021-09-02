mod model;
mod controller;
mod view;
use druid::{PlatformError};

fn main() -> Result<(), PlatformError>{
    let field = model::create_field();
    view::launch_app(field)?;
    Ok(())
}
