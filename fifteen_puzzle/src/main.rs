mod model;
mod controller;
mod view;
use druid::{AppLauncher, PlatformError,WindowDesc};

fn main() -> Result<(), PlatformError>{
    let field = model::create_field();
    println!("{}", field.to_string());
    AppLauncher::with_window(WindowDesc::new(||{view::build_ui()})).use_simple_logger().launch(field)?;

    Ok(())
}
