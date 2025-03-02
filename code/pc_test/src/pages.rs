#[path = "../../src/pages/pages.rs"]
pub mod pages;

#[path = "../../src/pages/state.rs"]
pub mod state;

#[path = "../../src/pages/core_pages.rs"]
mod core_pages;

#[path = "../../src/pages/snake.rs"]
mod snake;

#[path = "../../src/pages/video.rs"]
mod video;

#[path = "../../src/pages/vid_frames.rs"]
mod vid_frames;

use crate::shared_data::SharedData;
use crate::pages::state::PageState;

pub struct Pages
{
    curr_page: fn(&mut SharedData) -> (),
    curr_state: PageState,  
}
