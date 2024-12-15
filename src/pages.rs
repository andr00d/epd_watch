pub mod pages;
mod core_pages;

pub struct Pages<'a>
{
    curr_page: &'a dyn Fn() -> (), 
}
