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

#[status_script(agent = "eflame", status = FIGHTER_EFLAME_STATUS_KIND_SPECIAL_N_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_n_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_n_hold")),L2CValue::new_int(hash40("special_air_n_hold")),L2CValue::new_bool(false));
    fighter.sub_set_ground_correct_by_situation(L2CValue::new_bool(true));
    WorkModule::off_flag(module_accessor,*FIGHTER_EFLAME_STATUS_SPECIAL_N_WORK_FLAG_SPEED_CHANGE_HOLD_END_CHANGED);
    WorkModule::off_flag(module_accessor,*FIGHTER_EFLAME_STATUS_SPECIAL_N_WORK_FLAG_SPEED_CHANGE_HOLD_END_REQUEST);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_hold_main_loop as *const () as _))
}

unsafe fn special_n_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let keep_frame_max = WorkModule::get_param_int(module_accessor,hash40("param_special_n"),hash40("keep_frame_max")) as f32;
    let mut unk = false;
    if ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_SPECIAL) == false {
        unk = true;
    }
    if fighter.global_table[0xe].get_f32() >= keep_frame_max {
        unk = true;
    }
    if unk == false {
        fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_n_hold")),L2CValue::new_int(hash40("special_air_n_hold")),L2CValue::new_bool(true));
        fighter.sub_set_ground_correct_by_situation(L2CValue::new_bool(true));
        fighter.sub_exec_special_start_common_kinetic_setting(L2CValue::new_int(hash40("param_special_n")));
        if WorkModule::is_flag(module_accessor,*FIGHTER_EFLAME_STATUS_SPECIAL_N_WORK_FLAG_SPEED_CHANGE_HOLD_END_CHANGED) == false {
            if WorkModule::is_flag(module_accessor,*FIGHTER_EFLAME_STATUS_SPECIAL_N_WORK_FLAG_SPEED_CHANGE_HOLD_END_REQUEST) {
                if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
                    let hold_end_accel_y_mul = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),hash40("hold_end_accel_y_mul"));
                    let hold_end_max_speed_y_mul = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),hash40("hold_end_max_speed_y_mul"));
                    let mut air_accel_y = WorkModule::get_param_float(module_accessor,hash40("air_accel_y"),0) * -1.0;
                    if hold_end_accel_y_mul != 0.0 {
                        air_accel_y *= hold_end_accel_y_mul;
                    }
                    let mut air_speed_y_stable = WorkModule::get_param_float(module_accessor,hash40("air_speed_y_stable"),0);
                    if hold_end_max_speed_y_mul != 0.0 {
                        air_speed_y_stable *= hold_end_max_speed_y_mul;
                    }
                    fighter.clear_lua_stack();
                    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,air_accel_y);
                    sv_kinetic_energy::set_accel(fighter.lua_state_agent);
                    fighter.clear_lua_stack();
                    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,air_speed_y_stable);
                    sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
                    sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
                    WorkModule::on_flag(module_accessor,*FIGHTER_EFLAME_STATUS_SPECIAL_N_WORK_FLAG_SPEED_CHANGE_HOLD_END_CHANGED);
                    WorkModule::off_flag(module_accessor,*FIGHTER_EFLAME_STATUS_SPECIAL_N_WORK_FLAG_SPEED_CHANGE_HOLD_END_REQUEST);

                }
            }
        }
        if ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_ATTACK) {
            change_aegis(fighter,*FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_N_END,Type::NORMAL);
        }
    }
    else {
        let keep_frame_2rot = WorkModule::get_param_int(module_accessor,hash40("param_special_n"),hash40("keep_frame_2rot")) as f32;
        let keep_frame_3rot = WorkModule::get_param_int(module_accessor,hash40("param_special_n"),hash40("keep_frame_3rot")) as f32;
        if fighter.global_table[0xe].get_f32() < keep_frame_3rot {
            if fighter.global_table[0xe].get_f32() < keep_frame_2rot {
                WorkModule::set_int(module_accessor,1,*FIGHTER_EFLAME_STATUS_SPECIAL_N_WORK_INT_ROTATE_NUM);
            }
            else {
                WorkModule::set_int(module_accessor,2,*FIGHTER_EFLAME_STATUS_SPECIAL_N_WORK_INT_ROTATE_NUM);
            }
        }
        else {
            WorkModule::set_int(module_accessor,3,*FIGHTER_EFLAME_STATUS_SPECIAL_N_WORK_INT_ROTATE_NUM);
        }
        let ratio = fighter.global_table[0xe].get_f32()/keep_frame_max;
        let finish_attack_power_rate = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),hash40("finish_attack_power_rate"));
        let lerp_num = fighter.lerp(L2CValue::new_num(1.0),L2CValue::new_num(finish_attack_power_rate),L2CValue::new_num(ratio)).get_f32() + 0.0;
        WorkModule::set_float(module_accessor,lerp_num,*FIGHTER_EFLAME_STATUS_SPECIAL_N_WORK_FLOAT_ATTACK_MUL);
        fighter.change_status(FIGHTER_EFLAME_STATUS_KIND_SPECIAL_N_ATTACK.into(),false.into());
    }
    return L2CValue::I32(0)
}

unsafe fn special_n_kinetics_setup(fighter: &mut L2CFighterCommon) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let speed_x = KineticModule::get_sum_speed_x(module_accessor,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
    let mut unk1 = 0.0;
    let mut unk2 = 0.0;
    let mut unk3 = 0.0;
    if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_AIR_STOP);
        unk1 = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),0x186d9e2da0u64);
        unk2 = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),0x22f517d47cu64);
        unk3 = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),0x16c9d2bc49u64);
        let unk4 = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),0x16e109c926u64) * -1.0;
        let unk5 = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),0x1ae9a361a9u64);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,*ENERGY_GRAVITY_RESET_TYPE_GRAVITY,0.0,0.0,0.0,0.0,0.0);
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,speed_y);
        sv_kinetic_energy::set_speed(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,unk4);
        sv_kinetic_energy::set_accel(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,unk5);
        sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
        sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,*ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,0.0,0.0,0.0,0.0,0.0);
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    }
    else {
        unk1 = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),0x1884fd8895u64);
        unk2 = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),0x221c747149u64);
        unk3 = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),0x1620b1197cu64);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,*ENERGY_CONTROLLER_RESET_TYPE_FREE,0.0,0.0,0.0,0.0,0.0);
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    }
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    if unk2 > speed_x {
        unk2 = speed_x;
    }
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,unk2,0.0);
    sv_kinetic_energy::set_speed(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,unk3,0.0);
    sv_kinetic_energy::set_brake(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,speed_x,0.0);
    sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
    sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,unk1,0.0);
    sv_kinetic_energy::controller_set_accel_x_add(fighter.lua_state_agent);
}

#[status_script(agent = "eflame", status = FIGHTER_EFLAME_STATUS_KIND_SPECIAL_N_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_n_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if LIST.lock().unwrap().list[entry_id].change {
        let frame = LIST.lock().unwrap().list[entry_id].frame * 0.67;
        let keep_frame_2rot = WorkModule::get_param_int(module_accessor,hash40("param_special_n"),hash40("keep_frame_2rot")) as f32;
        let keep_frame_3rot = WorkModule::get_param_int(module_accessor,hash40("param_special_n"),hash40("keep_frame_3rot")) as f32;
        if frame < keep_frame_3rot {
            if frame < keep_frame_2rot {
                WorkModule::set_int(module_accessor,1,*FIGHTER_EFLAME_STATUS_SPECIAL_N_WORK_INT_ROTATE_NUM);
            }
            else {
                WorkModule::set_int(module_accessor,2,*FIGHTER_EFLAME_STATUS_SPECIAL_N_WORK_INT_ROTATE_NUM);
            }
        }
        else {
            WorkModule::set_int(module_accessor,3,*FIGHTER_EFLAME_STATUS_SPECIAL_N_WORK_INT_ROTATE_NUM);
        }
        let change = Change::new(0,0.0,0.0,-1,false,-1,Type::NONE);
        LIST.lock().unwrap().update_list(change,entry_id);
    }
    match WorkModule::get_int(module_accessor,*FIGHTER_EFLAME_STATUS_SPECIAL_N_WORK_INT_ROTATE_NUM) {
        2 => fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_n2")),L2CValue::new_int(hash40("special_air_n2")),L2CValue::new_bool(false)),
        3 => fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_n3")),L2CValue::new_int(hash40("special_air_n3")),L2CValue::new_bool(false)),
        _ => fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_n1")),L2CValue::new_int(hash40("special_air_n1")),L2CValue::new_bool(false)),
    };
    fighter.sub_set_ground_correct_by_situation(L2CValue::new_bool(true));
    special_n_kinetics_setup(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_attack_main_loop as *const () as _))
}

unsafe fn special_n_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        fighter.change_status(FIGHTER_EFLAME_STATUS_KIND_SPECIAL_N_END.into(),false.into());
    }
    let unk1 = match WorkModule::get_int(module_accessor,*FIGHTER_EFLAME_STATUS_SPECIAL_N_WORK_INT_ROTATE_NUM) {
        2 => fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_n2")),L2CValue::new_int(hash40("special_air_n2")),L2CValue::new_bool(true)),
        3 => fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_n3")),L2CValue::new_int(hash40("special_air_n3")),L2CValue::new_bool(true)),
        _ => fighter.sub_change_motion_by_situation(L2CValue::new_int(hash40("special_n1")),L2CValue::new_int(hash40("special_air_n1")),L2CValue::new_bool(true)),
    };
    fighter.sub_set_ground_correct_by_situation(L2CValue::new_bool(true));
    let mut unk = 0.0;
    if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
        unk = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),0x22f517d47cu64);
    }
    else {
        unk = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),0x221c747149u64);
    }
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,unk,0.0);
    sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
    sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
    if unk1.get_bool() {
        special_n_kinetics_setup(fighter);
    }
    return L2CValue::I32(0)
}