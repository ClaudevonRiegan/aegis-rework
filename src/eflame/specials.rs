use smash::lib::{L2CValue,L2CAgent,lua_const::*};
use smash::lua2cpp::{L2CAgentBase,L2CFighterCommon,L2CFighterBase};
use smash::phx::*;
use smash::hash40;
use smash::app::lua_bind::*;
use smash::app::*;
use smash_script::macros::*;
use smash_script::lua_args;
use smashline::*;
use crate::FIGHTER_MANAGER;
use crate::eflame::*;

#[status_script(agent = "eflame", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    fighter.sub_set_special_start_common_kinetic_setting(L2CValue::new_int(hash40("param_special_s")));
    if WorkModule::is_flag(module_accessor,*FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_SPECIAL_S_FLICK) == false {
        fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_s")),L2CValue::new_int(hash40("special_air_s")),L2CValue::new_bool(false));
    }
    else {
        fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_s_flick")),L2CValue::new_int(hash40("special_air_s_flick")),L2CValue::new_bool(false));
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_main_loop as *const () as _))
}

unsafe fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if CancelModule::is_enable_cancel(module_accessor) == false
    || (fighter.sub_wait_ground_check_common(L2CValue::new_bool(false)).get_bool() == false
    && fighter.sub_air_check_fall_common().get_bool() == false) {
        fighter.sub_exec_special_start_common_kinetic_setting(L2CValue::new_int(hash40("param_special_s")));
        if WorkModule::is_flag(module_accessor,*FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_SPECIAL_S_FLICK) == false {
            fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_s")),L2CValue::new_int(hash40("special_air_s")),L2CValue::new_bool(true));
        }
        else {
            fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_s_flick")),L2CValue::new_int(hash40("special_air_s_flick")),L2CValue::new_bool(true));
        }
        if ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_ATTACK)
        && [hash40("fly_l"),hash40("fly_r"),hash40("fly_flick_l"),hash40("fly_flick_r")].contains(&ArticleModule::motion_kind(module_accessor,*FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD,ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))) {
            change_aegis(fighter,*FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_FORWARD,Type::NORMAL);
        }
        if MotionModule::is_end(module_accessor) {
            if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
            }
        }
    }
    else {
        if fighter.sub_wait_ground_check_common(L2CValue::new_bool(false)).get_bool()
        && fighter.sub_air_check_fall_common().get_bool() {
            return L2CValue::I32(1)
        }
    }
    return L2CValue::I32(0)
}