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
use crate::switch::*;

const FIGHTER_ELFAME_INSTANCE_WORK_ID_FLOAT_SPEED_Y: i32 = 0x4c;

#[status_script(agent = "eflame", status = FIGHTER_EFLAME_STATUS_KIND_SPECIAL_HI_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_hi_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("special_air_hi_jump"),0.0,1.0,false,0.0,false,false);
    KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_MOTION_AIR);
    let jump_speed_mul = WorkModule::get_param_float(module_accessor,hash40("param_special_hi"),hash40("jump_speed_mul"));
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_MOTION,jump_speed_mul);
    sv_kinetic_energy::set_speed_mul(fighter.lua_state_agent);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_jump_main_loop as *const () as _))
}

unsafe fn special_hi_jump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return L2CValue::I32(0)
    }
    if MotionModule::is_end(module_accessor) == false {
        let frame = MotionModule::frame(module_accessor).ceil();
        let end_frame = MotionModule::end_frame(module_accessor);
        if frame >= end_frame {
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
            let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent) + 0.0;
            WorkModule::set_float(module_accessor,speed_y,FIGHTER_ELFAME_INSTANCE_WORK_ID_FLOAT_SPEED_Y);
        }
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            if WorkModule::is_flag(module_accessor,*FIGHTER_EFLAME_STATUS_SPECIAL_HI_FLAG_ENABLE_GROUND) {
                WorkModule::off_flag(module_accessor,*FIGHTER_EFLAME_STATUS_SPECIAL_HI_FLAG_ENABLE_GROUND);
                fighter.change_status(FIGHTER_EFLAME_STATUS_KIND_SPECIAL_HI_END.into(),false.into());
            }
        }
        if WorkModule::is_flag(module_accessor,*FIGHTER_EFLAME_STATUS_SPECIAL_HI_FLAG_START_CONTROL) {
            WorkModule::off_flag(module_accessor,*FIGHTER_EFLAME_STATUS_SPECIAL_HI_FLAG_START_CONTROL);
            KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,*ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,0.0,0.0,0.0,0.0,0.0);
            sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
            let jump_speed_x_mul = WorkModule::get_param_float(module_accessor,hash40("param_special_hi"),hash40("jump_speed_x_mul"));
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,jump_speed_x_mul);
            sv_kinetic_energy::mul_x_speed_max(fighter.lua_state_agent);
            let jump_speed_x_max_mul = WorkModule::get_param_float(module_accessor,hash40("param_special_hi"),hash40("jump_speed_x_max_mul"));
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,jump_speed_x_max_mul);
            sv_kinetic_energy::mul_x_accel_mul(fighter.lua_state_agent);
        }
        if WorkModule::is_flag(module_accessor,*FIGHTER_EFLAME_STATUS_SPECIAL_HI_FLAG_END_CONTROL) {
            WorkModule::off_flag(module_accessor,*FIGHTER_EFLAME_STATUS_SPECIAL_HI_FLAG_END_CONTROL);
            KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        }
        if ControlModule::check_button_on_trriger(module_accessor,*CONTROL_PAD_BUTTON_ATTACK) {
            change_aegis(fighter,*FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_ATTACK2,Type::SPECIAL);
        }
        else if ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_ATTACK) {
            change_aegis(fighter,*FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_ATTACK1,Type::SPECIAL);
        }
    }
    else {
        fighter.change_status(FIGHTER_EFLAME_STATUS_KIND_SPECIAL_HI_LOOP.into(),false.into());
    }
    return L2CValue::I32(0)
}

#[status_script(agent = "eflame", status = FIGHTER_EFLAME_STATUS_KIND_SPECIAL_HI_LOOP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_hi_loop_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionModule::change_motion(module_accessor,Hash40::new("special_air_hi_fall"),0.0,1.0,false,0.0,false,false);
    let mut speed_y = WorkModule::get_float(module_accessor,FIGHTER_ELFAME_INSTANCE_WORK_ID_FLOAT_SPEED_Y);
    if speed_y <= 0.0 {
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
        speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
    }
    KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_MOTION_FALL);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,speed_y);
    sv_kinetic_energy::set_speed(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,speed_y.abs());
    sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
    sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_loop_main_loop as *const () as _))
}

unsafe fn special_hi_loop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if fighter.sub_transition_group_check_air_cliff().get_bool() == false {
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
        let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
        if speed_y <= 0.0 {
            EFFECT_FOLLOW(fighter, Hash40::new("eflame_sword_beam_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,-8.0);
            sv_kinetic_energy::set_speed(fighter.lua_state_agent);
            sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
            sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
        }
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_EFLAME_STATUS_KIND_SPECIAL_HI_END.into(),false.into());
        }
    }
    return L2CValue::I32(0)
}
