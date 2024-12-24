#[path = "../pages/pages.rs"]
pub mod pages;

#[path = "../pages/core_pages.rs"]
mod core_pages;

use crate::sharedData::SharedData;
use crate::io::Io;

pub struct Pages
{
    curr_page: fn(&mut SharedData) -> (), 
}
