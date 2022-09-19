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

#[status_script(agent = "elight", status = FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_LW_OUT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_lw_out(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status = LIST.lock().unwrap().list[entry_id].status;
    let type_atk = LIST.lock.unwrap().list[entry_id].type_atk;
    if status > -1 {
        if type_atk == Type::SPECIAL {
            let change = Change::new(0,0.0,0.0,-1,false,-1,Type::NONE);
            LIST.lock().unwrap().update_list(change,entry_id);
        }
        fighter.change_status(L2CValue::I32(status),false.into());
        return L2CValue::I32(1)
    }
    else {
        original!(fighter)
    }
}
