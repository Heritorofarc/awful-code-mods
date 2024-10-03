#![feature(lazy_cell, ptr_sub_ptr)]
use unity::{prelude::*};
use engage::gamedata::unit::Unit;
use engage::gamedata::JobData;
use engage::mess::Mess;

#[unity::class("App", "ClassChangeJobMenu.ClassChangeJobMenuItem")]
pub struct ClassChangeJobMenuItem {
    pub job: &'static JobData,
    junk: [u8; 0x10],
    pub set_attribute: u8,
}

//#[unity::hook("App", "ClassChangeJobMenu", "ClassChangeJobMenuItem")]
#[skyline::hook(offset=0x019c74a0)]
pub fn hide_jobselect_all(this: ClassChangeJobMenuItem, method_info: OptionalMethod) -> u8 {
        call_original!(this, method_info)
}

#[skyline::main(name = "hooks")]
pub fn main() {
    skyline::install_hook!(hide_jobselect_all);
}
