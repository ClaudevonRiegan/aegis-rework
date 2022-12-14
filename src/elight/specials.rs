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
use crate::elight::*;
use crate::switch::*;

#[status_script(agent = "elight", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_s_start")),L2CValue::new_int(hash40("special_air_s_start")),L2CValue::new_bool(false));
    fighter.sub_set_ground_correct_by_situation(L2CValue::new_bool(true));
    fighter.sub_set_special_start_common_kinetic_setting(L2CValue::new_int(hash40("param_special_s")));
    WorkModule::off_flag(module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_SET_HUD_OFF);
    WorkModule::off_flag(module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_TILT);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_main_loop as *const () as _))
}

unsafe fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) == false {
        fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_s_start")),L2CValue::new_int(hash40("special_air_s_start")),L2CValue::new_bool(true));
        fighter.sub_set_ground_correct_by_situation(L2CValue::new_bool(true));
        fighter.sub_exec_special_start_common_kinetic_setting(L2CValue::new_int(hash40("param_special_s")));
        if WorkModule::is_flag(module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_SET_HUD_OFF) {
            WorkModule::off_flag(module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_LOUPE);
            WorkModule::off_flag(module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_LOUPE_DAMAGE);
            WorkModule::off_flag(module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_CURSOR);
            WorkModule::off_flag(module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
            WorkModule::off_flag(module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_SET_HUD_OFF);
        }
    }
    else {
        fighter.change_status(FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_FORWARD.into(),false.into());
    }
    return L2CValue::I32(0)
}

#[status_script(agent = "elight", status = FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_FORWARD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_s_forward_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    WorkModule::off_flag(module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_CHECK_CLIFF);
    WorkModule::off_flag(module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_NEAR_CLIFF);
    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
        if GroundModule::is_touch(module_accessor,*GROUND_TOUCH_FLAG_DOWN as u32) {
            let pos = GroundModule::get_touch_normal_consider_gravity(module_accessor,*GROUND_TOUCH_FLAG_DOWN as u32);
            let length = sv_math::vec2_length(pos.x,pos.y);
            if length > 0.00001 {
                let angle = ((pos.y/pos.x).atan()).to_degrees().abs();
                if WorkModule::get_param_float(module_accessor,hash40("param_special_s"),0x11e7fad1adu64) < angle {
                    if PostureModule::lr(module_accessor) * pos.x < 0.0 {
                        fighter.set_situation(SITUATION_KIND_AIR.into());
                        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                        WorkModule::on_flag(module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_TILT);
                    }
                }
            }
        }
    }
    fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_s")),L2CValue::new_int(hash40("special_air_s")),L2CValue::new_bool(false));
    fighter.sub_set_ground_correct_by_situation(L2CValue::new_bool(false));
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    let speed = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
    if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_MOTION_AIR);
    }
    else {
        KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_MOTION);
    }
    let speed_x_mul = WorkModule::get_param_float(module_accessor,hash40("param_special_s"),hash40("speed_x_mul"));
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,speed,0.0);
    sv_kinetic_energy::set_speed(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_MOTION,speed_x_mul);
    sv_kinetic_energy::set_speed_mul(fighter.lua_state_agent);
    WorkModule::off_flag(module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_LOUPE);
    WorkModule::off_flag(module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_LOUPE_DAMAGE);
    WorkModule::off_flag(module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_CURSOR);
    WorkModule::off_flag(module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
    GroundModule::set_shape_flag(module_accessor,*GROUND_CORRECT_SHAPE_RHOMBUS_MODIFY_FLAG_FRONT_FIX as u16,true);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_forward_main_loop as *const () as _))
}

unsafe fn special_s_forward_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if (CancelModule::is_enable_cancel(module_accessor) == false
    || (fighter.sub_wait_ground_check_common(L2CValue::new_bool(false)).get_bool() == false
    && fighter.sub_air_check_fall_common().get_bool()))
    && fighter.sub_transition_group_check_air_cliff().get_bool() == false {
        if MotionModule::is_end(module_accessor) == false {
            if LIST.lock().unwrap().list[entry_id].change {
                MotionModule::set_frame(module_accessor,18.0,false);
                let change = Change::new(0,0.0,0.0,-1,false,-1,Type::NONE);
                LIST.lock().unwrap().update_list(change,entry_id);
            }
            let ground_cliff_stop_frame = WorkModule::get_param_int(module_accessor,hash40("param_special_s"),hash40("ground_cliff_stop_frame")) as f32;
            if fighter.global_table[0xe].get_f32() >= ground_cliff_stop_frame {
                let is_near_cliff_threshold = WorkModule::get_param_float(module_accessor,hash40("param_special_s"),hash40("is_near_cliff_threshold"));
                if WorkModule::is_flag(module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_CHECK_CLIFF)
                && WorkModule::is_flag(module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_NEAR_CLIFF) == false {
                    let num = PostureModule::scale(module_accessor) * is_near_cliff_threshold;
                    if GroundModule::is_ottotto(module_accessor,num) {
                        WorkModule::on_flag(module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_NEAR_CLIFF);
                    }
                }
            }
            else {
                let mut unk = true;
                if WorkModule::is_flag(module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_TILT) {
                    unk = false;
                    let air_fix_frame = WorkModule::get_param_int(module_accessor,hash40("param_special_s"),hash40("air_fix_frame")) as f32;
                    if fighter.global_table[0xe].get_f32() >= air_fix_frame {
                        WorkModule::off_flag(module_accessor,*FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_TILT);
                    }
                }
                if unk {
                    let motion = MotionModule::motion_kind(module_accessor);
                    let frame = MotionModule::frame(module_accessor);
                    let rate = MotionModule::rate(module_accessor);
                    if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND
                    || motion == hash40("special_s") {
                        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR
                        && motion != hash40("special_air_s") {
                            MotionModule::change_motion_inherit_frame(module_accessor,Hash40::new_raw(motion),frame,rate,0.0,true,false);
                        }
                    }
                    else {
                        MotionModule::change_motion_inherit_frame(module_accessor,Hash40::new_raw(motion),frame,rate,0.0,true,false);
                    }
                    fighter.sub_set_ground_correct_by_situation(L2CValue::new_bool(true));
                }
            }
            if fighter.global_table[0xe].get_f32() < 18.0
            && ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_ATTACK) {
                change_aegis(fighter,*FIGHTER_STATUS_KIND_SPECIAL_S,Type::SPECIAL);
            }
        }
        else {
            fighter.change_status(FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_END.into(),false.into());
        }
    }
    return L2CValue::I32(0)
}

#[status_script(agent = "elight", status = FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_s_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_s_end")),L2CValue::new_int(hash40("special_air_s_end")),L2CValue::new_bool(false));
    fighter.sub_set_ground_correct_by_situation(L2CValue::new_bool(true));
    fighter.sub_set_special_start_common_kinetic_setting(L2CValue::new_int(hash40("param_special_s")));
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_end_main_loop as *const () as _))
}

unsafe fn special_s_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if (CancelModule::is_enable_cancel(module_accessor) == false
    || (fighter.sub_wait_ground_check_common(L2CValue::new_bool(false)).get_bool() == false
    && fighter.sub_air_check_fall_common().get_bool()))
    && fighter.sub_transition_group_check_air_cliff().get_bool() == false {
        if MotionModule::is_end(module_accessor) == false {
            if fighter.global_table[0x17].get_i32() != *SITUATION_KIND_GROUND
            && fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
                let frame = MotionModule::frame(module_accessor);
                let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(module_accessor,Hash40::new("special_s_end"),true);
                if frame >=  cancel_frame {
                    fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(),false.into());
                }
                SoundModule::play_landing_se(module_accessor,Hash40::new("se_elight_landing01"));
            }
            fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_s_end")),L2CValue::new_int(hash40("special_air_s_end")),L2CValue::new_bool(true));
            fighter.sub_set_ground_correct_by_situation(L2CValue::new_bool(true));
            fighter.sub_exec_special_start_common_kinetic_setting(L2CValue::new_int(hash40("param_special_s")));
        }
        else {
            if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(),false.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
            }
        }
    }
    return L2CValue::I32(0)
}