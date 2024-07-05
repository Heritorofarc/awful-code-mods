#![feature(lazy_cell, ptr_sub_ptr)]
use unity::{prelude::*};
use engage::gamedata::unit::Unit;
    //mess::*,
    //gamedata::*
//};
//#[unity::hook("App", "HelpParamSetter", "SetBattleInfo")]
#[skyline::hook(offset=0x02160ac0)]
pub fn set_battle_info_hook(cock: u64, balls: u64, unit: &Unit, text_type: i32, text: &Il2CppString, method_info: OptionalMethod){
    //meepers code
    call_original!(cock, balls, unit, text_type, text, method_info); 
    if unit.force.unwrap().force_type != 0 && unit.force.unwrap().force_type != 3 { // If not Player/Absent
        println!("this unit is an outside force");
        return;
    }
    println!("this unit is the player force");
    let stat_index = match text_type {
            0 => 0,
            7 => 3,
            8 => 1,
            9 => 6,
            10 => 2,
            12 => 5,
            13 => 7,
            14 => 4,
            15 => 8,
            _ => -1,
        };
    if stat_index == -1 { return; }
    if stat_index != -1 { println!("this is a tracked stat"); }
    }
#[skyline::main(name = "hooks")]
pub fn main() {
    skyline::install_hooks!(set_battle_info_hook);
}
