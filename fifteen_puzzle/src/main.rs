mod model;
mod controller;
use druid::{AppLauncher, WindowDesc, Widget, PlatformError};
use druid::widget::{Label,Flex,Padding};


fn build_ui() -> impl Widget<()> {
    Padding::new(100.0,
        Flex::row().with_flex_child(
            Flex::column()
                .with_flex_child(Label::new("top left"), 1.0)
                .with_flex_child(Label::new("bottom left"), 1.0),
            1.0)
            .with_flex_child(Flex::column()
            .with_flex_child(Label::new("top right"), 1.0)
            .with_flex_child(Label::new("bottom right"), 1.0),
            1.0))
}
fn main() -> Result<(), PlatformError>{
    let field = model::create_field();
    AppLauncher::with_window(WindowDesc::new(||{build_ui()})).launch(())?;

    Ok(())
}
