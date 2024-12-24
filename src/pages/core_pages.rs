use crate::pages::Pages;
use crate::sharedData::SharedData;
use crate::display::font::Anchor;
use crate::io::Io;

impl Pages
{
    pub(super) fn menu_time(data: &mut SharedData) 
    {
        let time_str = data.io.get_time_str();
        let date_str = data.io.get_date_str();
        let dow_str = data.io.get_dow_str();

        data.display.text(&time_str, 100, 75, 10, Anchor::Center);
        let offset = (data.display.get_text_width(&time_str, 10) / 2) as u8;

        data.display.text(&dow_str, 100-offset, 50, 4, Anchor::Left);
        data.display.text(&date_str, 100+offset, 50, 4, Anchor::Right)
    }

    pub(super) fn menu_settings() 
    {

    }

    pub(super) fn menu_timer() 
    {

    }

    pub(super) fn menu_stopwatch() 
    {

    }
}