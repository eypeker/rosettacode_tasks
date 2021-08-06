use super::model;

use druid::{AppLauncher, PlatformError, Widget, WidgetExt, WindowDesc};
use druid::widget::{Label, Flex};


use super::model::{field::Tilegrid as Tg, point::Point};


fn get_str(val:u32)->String{
    match val {
        0 => " ".to_owned(),
        k => k.to_string(),
    }
}


fn get_label(x:usize, y:usize)-> impl Widget<Tg>{

    return Label::new(move |data:&Tg, _env:&_| get_str(data.get_field()[x.clone()][y.clone()]))
        .fix_height(100.0).fix_width(100.0)
        .align_horizontal(druid::UnitPoint::CENTER)
        .align_vertical(druid::UnitPoint::CENTER)
        .center()
        //.layout(ctx, bc, data, env)
        .border(druid::Color::grey8(255),2.0 )
        .on_click(move|_ctx, data:&mut Tg,_env| data.move_tile(Point::new(x as u8,y as u8)))
}

pub fn build_ui() -> impl Widget<Tg>{
    Flex::column()
        .with_flex_child(Flex::row()
            .with_flex_child(get_label(0,0), 1.0)
            .with_flex_child(get_label(0,1), 1.0)
            .with_flex_child(get_label(0,2), 1.0)
            .with_flex_child(get_label(0,3), 1.0)
            , 1.0)
            .with_flex_child(Flex::row()
            .with_flex_child(get_label(1,0), 1.0)
            .with_flex_child(get_label(1,1), 1.0)
            .with_flex_child(get_label(1,2), 1.0)
            .with_flex_child(get_label(1,3), 1.0)
            , 1.0)
            .with_flex_child(Flex::row()
            .with_flex_child(get_label(2,0), 1.0)
            .with_flex_child(get_label(2,1), 1.0)
            .with_flex_child(get_label(2,2), 1.0)
            .with_flex_child(get_label(2,3), 1.0)
            , 1.0)
            .with_flex_child(Flex::row()
            .with_flex_child(get_label(3,0), 1.0)
            .with_flex_child(get_label(3,1), 1.0)
            .with_flex_child(get_label(3,2), 1.0)
            .with_flex_child(get_label(3,3), 1.0)
            , 1.0)
}

pub fn launch_app(field:model::Tg) -> Result<(), PlatformError>{
    AppLauncher::with_window(
        WindowDesc::new(||{build_ui()})
            .title("Fifteen Puzzle")
            .window_size((400.0, 400.0))
            .resizable(false)
    ).use_simple_logger().launch(field)?;
    Ok(())
}
