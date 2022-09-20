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

#[status_script(agent = "elight", status = FIGHTER_STATUS_KIND_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let instance = &LIST.lock().unwrap().list[entry_id];
    if instance.change {
        MotionModule::change_motion(module_accessor,Hash40::new_raw(instance.motion),instance.frame,instance.rate,false,0.0,false,false);
        fighter.sub_shift_status_main(L2CValue::Ptr(attack_main_loop as *const () as _))
    }
    else {
        fighter.status_Attack()
    }
}

unsafe fn attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if CancelModule::is_enable_cancel(module_accessor) {
        let change = Change::new(0,0.0,0.0,-1,false,-1,Type::NONE);
        LIST.lock().unwrap().update_list(change,entry_id);
    }
    if MotionModule::is_end(module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
    }
    return L2CValue::I32(0)
}

#[status_script(agent = "elight", status = FIGHTER_STATUS_KIND_ATTACK_S3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn attack_s3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let instance = &LIST.lock().unwrap().list[entry_id];
    if instance.change {
        MotionModule::change_motion(module_accessor,Hash40::new_raw(instance.motion),instance.frame,instance.rate,false,0.0,false,false);
        fighter.sub_shift_status_main(L2CValue::Ptr(attack_main_loop as *const () as _))
    }
    else {
        fighter.status_AttackS3()
    }
}

#[status_script(agent = "elight", status = FIGHTER_STATUS_KIND_ATTACK_HI3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn attack_hi3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let instance = &LIST.lock().unwrap().list[entry_id];
    if instance.change {
        MotionModule::change_motion(module_accessor,Hash40::new_raw(instance.motion),instance.frame,instance.rate,false,0.0,false,false);
        fighter.sub_shift_status_main(L2CValue::Ptr(attack_main_loop as *const () as _))
    }
    else {
        fighter.status_AttackHi3()
    }
}

#[status_script(agent = "elight", status = FIGHTER_STATUS_KIND_ATTACK_LW3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn attack_lw3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let instance = &LIST.lock().unwrap().list[entry_id];
    if instance.change {
        MotionModule::change_motion(module_accessor,Hash40::new_raw(instance.motion),instance.frame,instance.rate,false,0.0,false,false);
        fighter.sub_shift_status_main(L2CValue::Ptr(attack_main_loop as *const () as _))
    }
    else {
        fighter.status_AttackLw3()
    }
}

#[status_script(agent = "elight", status = FIGHTER_STATUS_KIND_ATTACK_S4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn attack_s4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    original!(fighter);
    let instance = &LIST.lock().unwrap().list[entry_id];
    if instance.change {
        MotionModule::set_frame(module_accessor,instance.frame,false);
        MotionModule::set_rate(module_accessor,instance.rate);
        if CancelModule::is_enable_cancel(module_accessor) {
            let change = Change::new(0,0.0,0.0,-1,false,-1,Type::NONE);
            LIST.lock().unwrap().update_list(change,entry_id);
        }
    }
    return L2CValue::I32(0)
}

#[status_script(agent = "elight", status = FIGHTER_STATUS_KIND_ATTACK_HI4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn attack_hi4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let instance = &LIST.lock().unwrap().list[entry_id];
    if instance.change {
        MotionModule::change_motion(module_accessor,Hash40::new_raw(instance.motion),instance.frame,instance.rate,false,0.0,false,false);
        fighter.sub_shift_status_main(L2CValue::Ptr(attack_main_loop as *const () as _))
    }
    else {
        fighter.status_AttackHi4()
    }
}

#[status_script(agent = "elight", status = FIGHTER_STATUS_KIND_ATTACK_LW4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn attack_lw4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let instance = &LIST.lock().unwrap().list[entry_id];
    if instance.change {
        MotionModule::change_motion(module_accessor,Hash40::new_raw(instance.motion),instance.frame,instance.rate,false,0.0,false,false);
        fighter.sub_shift_status_main(L2CValue::Ptr(attack_main_loop as *const () as _))
    }
    else {
        fighter.status_AttackLw4()
    }
}

#[status_script(agent = "elight", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let instance = &LIST.lock().unwrap().list[entry_id];
    if instance.change {
        ControlModule::set_attack_air_kind(module_accessor,instance.air_attack);
        fighter.sub_attack_air_common(L2CValue::new_bool(true));
        fighter.sub_shift_status_main(L2CValue::Ptr(attack_air_main_loop as *const () as _))
    }
    else {
        fighter.status_AttackAir()
    }
}

unsafe fn attack_air_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    fighter.status_AttackAir_Main();
    if CancelModule::is_enable_cancel(module_accessor) {
        let change = Change::new(0,0.0,0.0,-1,false,-1,Type::NONE);
        LIST.lock().unwrap().update_list(change,entry_id);
    }
    return L2CValue::I32(0)
}