mod model;
mod controller;
use druid::{AppLauncher, PlatformError, Widget, WidgetExt, WindowDesc};
use druid::widget::{Label, Flex, Padding};
use model::{field::Tilegrid as Tg, point::Point};


fn _build_ui() -> impl Widget<()> {
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

fn get_str(val:Option<u32>)->String{
    match val {
        Some(k) => k.to_string(),
        None => "".to_string()
    }
}

fn build_ui() -> impl Widget<Tg>{
    Flex::column()
        .with_flex_child(Flex::row()
            .with_flex_child(Label::new(|data:&Tg, _env:&_| data.get_field()[0][0].to_string())
            .on_click(move|_ctx, data:&mut Tg,_env| data.move_tile(Point::new(0,0))), 1.0)
            .with_flex_child(Label::new(|data:&Tg, _env:&_| data.get_field()[0][1].to_string())
            .on_click(move|_ctx, data:&mut Tg,_env| data.move_tile(Point::new(0,1))), 1.0)
            .with_flex_child(Label::new(|data:&Tg, _env:&_| data.get_field()[0][2].to_string())
            .on_click(move|_ctx, data:&mut Tg,_env| data.move_tile(Point::new(0,2))), 1.0)
            .with_flex_child(Label::new(|data:&Tg, _env:&_| data.get_field()[0][3].to_string())
            .on_click(move|_ctx, data:&mut Tg,_env| data.move_tile(Point::new(0,3))), 1.0)
            , 1.0)
        .with_flex_child(Flex::row()
            .with_flex_child(Label::new(|data:&Tg, _env:&_| data.get_field()[1][0].to_string())
            .on_click(move|_ctx, data:&mut Tg,_env| data.move_tile(Point::new(1,0))), 1.0)
            .with_flex_child(Label::new(|data:&Tg, _env:&_| data.get_field()[1][1].to_string())
            .on_click(move|_ctx, data:&mut Tg,_env| data.move_tile(Point::new(1,1))), 1.0)
            .with_flex_child(Label::new(|data:&Tg, _env:&_| data.get_field()[1][2].to_string())
            .on_click(move|_ctx, data:&mut Tg,_env| data.move_tile(Point::new(1,2))), 1.0)
            .with_flex_child(Label::new(|data:&Tg, _env:&_| data.get_field()[1][3].to_string())
            .on_click(move|_ctx, data:&mut Tg,_env| data.move_tile(Point::new(1,3))), 1.0)
            , 1.0)
        .with_flex_child(Flex::row()
            .with_flex_child(Label::new(|data:&Tg, _env:&_| data.get_field()[2][0].to_string())
            .on_click(move|_ctx, data:&mut Tg,_env| data.move_tile(Point::new(2,0))), 1.0)
            .with_flex_child(Label::new(|data:&Tg, _env:&_| data.get_field()[2][1].to_string())
            .on_click(move|_ctx, data:&mut Tg,_env| data.move_tile(Point::new(2,1))), 1.0)
            .with_flex_child(Label::new(|data:&Tg, _env:&_| data.get_field()[2][2].to_string())
            .on_click(move|_ctx, data:&mut Tg,_env| data.move_tile(Point::new(2,2))), 1.0)
            .with_flex_child(Label::new(|data:&Tg, _env:&_| data.get_field()[2][3].to_string())
            .on_click(move|_ctx, data:&mut Tg,_env| data.move_tile(Point::new(2,3))), 1.0)
            , 1.0)
        .with_flex_child(Flex::row()
            .with_flex_child(Label::new(|data:&Tg, _env:&_| data.get_field()[3][0].to_string())
            .on_click(move|_ctx, data:&mut Tg,_env| data.move_tile(Point::new(3,0))), 1.0)
            .with_flex_child(Label::new(|data:&Tg, _env:&_| data.get_field()[3][1].to_string())
            .on_click(move|_ctx, data:&mut Tg,_env| data.move_tile(Point::new(3,1))), 1.0)
            .with_flex_child(Label::new(|data:&Tg, _env:&_| data.get_field()[3][2].to_string())
            .on_click(move|_ctx, data:&mut Tg,_env| data.move_tile(Point::new(3,2))), 1.0)
            .with_flex_child(Label::new(|data:&Tg, _env:&_| data.get_field()[3][3].to_string())
            .on_click(move|_ctx, data:&mut Tg,_env| data.move_tile(Point::new(3,3))), 1.0)
            , 1.0)
    
    
}


fn main() -> Result<(), PlatformError>{
    let field = model::create_field();
    println!("{}", field.to_string());
    AppLauncher::with_window(WindowDesc::new(||{build_ui()})).use_simple_logger().launch(field)?;


    Ok(())
}
