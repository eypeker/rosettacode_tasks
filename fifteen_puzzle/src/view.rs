use druid::{AppLauncher, PlatformError, Widget, WidgetExt, WindowDesc};
use druid::widget::{Label, Flex, Padding};

use super::model::{field::Tilegrid as Tg, point::Point};

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


fn get_label(x:usize, y:usize)-> impl Widget<Tg>{

    return Label::new(move |data:&Tg, _env:&_| data.get_field()[x.clone()][y.clone()].to_string())
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
