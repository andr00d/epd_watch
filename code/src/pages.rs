pub mod pages;
pub mod state;
mod core_pages;
mod snake;
mod video;
mod vid_frames;

use crate::shared_data::SharedData;
use crate::pages::state::PageState;

pub struct Pages
{
    curr_page: fn(&mut SharedData) -> (),
    curr_state: PageState,  
}
