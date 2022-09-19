use smash::lib::{L2CValue,lua_const::*};
use smash::lua2cpp::L2CFighterCommon;
use smash::hash40;
use smash::app::lua_bind::*;
use smash::app::*;
use smashline::*;
use skyline::nn::ro::LookupSymbol;

mod speciallw;
use speciallw::*;
mod specials;
use specials::*;
mod specialhi;
use specialhi::*;
mod specialn;
use specialn::*;
use crate::FIGHTER_MANAGER;

extern "C" {
    #[link_name = "\u{1}_ZN3app25FighterSpecializer_ELight22kirby_esword_update_lrERNS_21FighterModuleAccessorE"]
    pub fn FighterSpecializer_Elight__kirby_sword_update_lr(module_accessor: *mut FighterModuleAccessor);
}

/*#[smashline::fighter_frame(agent = FIGHTER_KIND_ELIGHT)]
pub fn elight(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        let status = StatusModule::status_kind(module_accessor);
        let entry_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if [*FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status)
        || ([*FIGHTER_STATUS_KIND_ATTACK].contains(&status) && MotionModule::motion_kind(module_accessor) == hash40("attack_13")) {
            if AttackModule::is_infliction_status(module_accessor,*COLLISION_KIND_MASK_HIT)
            && ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_SPECIAL) {
                change_aegis(fighter,status,Type::NORMAL);
            }
        }
        if status == *FIGHTER_STATUS_KIND_ATTACK_AIR {
            if AttackModule::is_infliction_status(module_accessor,*COLLISION_KIND_MASK_HIT)
            && ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_SPECIAL) {
                change_aegis(fighter,status,Type::AERIAL);
            }
        }
        if status == CURR_STATUS[entry_id] && CHANGE[entry_id] {
            if MOTION[entry_id] == hash40("none") {
                MotionModule::set_frame(module_accessor,FRAME[entry_id],false);
                MotionModule::set_rate(module_accessor,RATE[entry_id]);
            }
            else {
                MotionModule::set_frame(module_accessor,FRAME[entry_id],false);
                MotionModule::set_rate(module_accessor,RATE[entry_id]);
                ControlModule::set_attack_air_kind(module_accessor,MOTION[entry_id] as i32);
            }
            CHANGE[entry_id] = false;
            CURR_STATUS[entry_id] = -1;
        }
    }
}*/

pub fn install() {
    unsafe {
        LookupSymbol(
            &mut FIGHTER_MANAGER,
            "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
            .as_bytes()
            .as_ptr(),
        );
    }
    //install_agent_frame!(elight);
    install_status_scripts!(
        special_lw_out,
        special_s_main,
        special_s_forward_main,
        special_s_end_main,
        special_hi_jump_main,
        special_n_main,
        special_n_hold_main,
        special_n_end_main,
    );
}