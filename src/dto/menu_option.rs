#[non_exhaustive]
pub struct MenuOption;

impl MenuOption {
    pub const SHOW_MENU : i32 = 0;
    pub const LIST: i32 = 1;
    pub const ADD: i32 = 2;
    pub const EDIT : i32 = 3;
    pub const DELETE: i32 = 4;
    pub const GET: i32 = 5;
    pub const EXIT: i32 = 6;
}