#![feature(lazy_cell, ptr_sub_ptr)]
use unity::{prelude::*};
use engage::gamedata::unit::Unit;
use engage::mess::Mess;
//Address is offset by 7100000000 in Ghidra. Use Ray's project to find this function's address
//App.HelpParamSetter$$SetBattleInfo
#[skyline::hook(offset=0x02160ac0)]
//#[unity::hook("App", "HelpParamSetter", "SetBattleInfo")] <<< another way to hook, uncomment if you want to use this instead.
//Function uses 6 parameters. (what these [vvvv] are called) when hooking into a vanilla function [^^^^] make sure you match its parameters, visible in Ghidra.
pub fn set_battle_info_hook(cock: u64, balls: u64, unit: &Unit, text_type: i32, text: &Il2CppString, method_info: OptionalMethod){
    //this calls the original function to run, which displays the tooltip text as normal.
    call_original!(cock, balls, unit, text_type, text, method_info);
    //if not part of blue party or in somniel (eg, an enemy unit), end code -- displays text like vanilla as nothing gets added
    if unit.force.unwrap().force_type != 0 && unit.force.unwrap().force_type != 3 {
        return;
    }
    //stat_index is defined here. Stealing the i32 from when it's checked in [call_original!] above and comparing it. 
    let stat_index = match text_type {
        //It's actually arranged backwards, which may be confusing. Left numbers are [text_type] turned into [via =>] [stat_index]. Each right number represents a different stat. 0 HP, 1 Str, etc.
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
    //if stat index is an actual stat, print text into the terminal
    if stat_index != -1 { println!("printing [personal+class] growths"); }
    else { return; }
    //defines the total growth rate. Boolean (true/false) decides whether to ignore adding class growths. We want the combined growths, so [false]
    let grow = unit.get_capability_grow(stat_index, false);
    if grow > 0  {
        //defines our modified text. {original help text}\newline{string: "Stat Growths"}: {growthrate}
        let new_text = format!("{}\n{}: {}", 
            text.get_string().unwrap(), 
            Mess::get("MID_GAMESTART_GROWMODE_SELECT_TITLE").get_string().unwrap() , 
            grow);
        //when [if grow > 0] is fulfilled, modify the text behind the scenes and then output it with the additions specified (growth display)
        call_original!(cock, balls, unit, text_type, new_text.into(), method_info); 
    }
}
#[skyline::main(name = "hooks")]
pub fn main() {
    skyline::install_hooks!(set_battle_info_hook);
}
