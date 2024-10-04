#![feature(lazy_cell, ptr_sub_ptr)]
use unity::{prelude::*};
use engage::gamedata::unit::Unit;
use engage::gamedata::JobData;
use engage::mess::Mess;
use engage::gamedata::WeaponMask;

#[unity::class("App", "ClassChange.ChangeJobData")]
pub struct ChangeJobData {
    pub job: &'static JobData,
    pub job_weapon_mask: &'static WeaponMask,
    pub original_job_weapon_mask: &'static WeaponMask,
    pub proof_type: i32, 
    __: i32,
    pub cost_level: &'static Il2CppString,
    pub is_enough_level: bool,
    pub junk: [u8; 7],
    pub cost_weapon_mask: &'static WeaponMask,
    pub equippable_weapon_mask: &'static WeaponMask,
    pub enough_item: bool,
    pub is_gender: bool,
    pub is_default_job: bool,
}

//#[unity::class("App", "ClassChange.ChangeJobData")]
#[skyline::hook(offset=0x019c6700)]
pub fn compare_sid_jid(this: &mut ChangeJobData, unit: &Unit, method_info: OptionalMethod) -> bool {
    //let sid_append: &Il2CppString = format!("SID_{}", this.job.jid.get_string().unwrap()).into();
    let as_normal = call_original!(this, unit, method_info);
    if this.is_default_job == false {
        this.is_gender = false;
        return false;
    }
    else {
        return as_normal
}
}

#[skyline::main(name = "hooks")]
pub fn main() {
    skyline::install_hook!(compare_sid_jid);
}
