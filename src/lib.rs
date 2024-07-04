#![feature(lazy_cell, ptr_sub_ptr)]
use unity::{prelude::*};
use engage::gamedata::unit::Unit;
    //mess::*,
    //gamedata::*
//};
//App.HelpParamSetter$$SetBattleInfo
#[skyline::hook(offset=0x02160ac0)]
pub fn set_battle_info_hook(cock: u64, balls: u64, unit: &Unit, text_type: i32, text: &Il2CppString, method_info: OptionalMethod){
    //meepers code
    call_original!(cock, balls, unit, text_type, text, method_info); 
    if unit.force.unwrap().force_type != 0 && force != 3 { // If not Player/Absent
        println!("this plugin is doing something");
        return; 
    }
skyline::install_hooks!(set_battle_info_hook);
}