use smash::lib::{L2CValue,lua_const::*};
use smash::lua2cpp::L2CFighterCommon;
use smash::hash40;
use smash::app::lua_bind::*;
use smashline::*;
use skyline::nn::ro::LookupSymbol;

mod normals;
use normals::*;
mod speciallw;
use speciallw::*;
mod specialhi;
use specialhi::*;
mod specials;
use specials::*;
mod specialn;
use specialn::*;
use crate::FIGHTER_MANAGER;
use crate::switch::*;

#[smashline::fighter_frame(agent = FIGHTER_KIND_EFLAME)]
pub fn eflame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        let status = StatusModule::status_kind(module_accessor);
        let entry_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if [*FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status)
        || ([*FIGHTER_STATUS_KIND_ATTACK].contains(&status) && MotionModule::motion_kind(module_accessor) == hash40("attack_13")) {
            if AttackModule::is_infliction_status(module_accessor,*COLLISION_KIND_MASK_HIT)
            && ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_SPECIAL)
            && CHANGE[entry_id] == false {
                CHANGE[entry_id] = true;
            }
        }
        if status == *FIGHTER_STATUS_KIND_ATTACK_AIR {
            if AttackModule::is_infliction_status(module_accessor,*COLLISION_KIND_MASK_HIT)
            && ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_SPECIAL)
            && CHANGE[entry_id] == false {
                CHANGE[entry_id] = true;
            }
        }
        if CHANGE[entry_id] {
            let mut box_count = 0;
            for i in 0..10 {
                if AttackModule::is_attack(module_accessor,i,false) {
                    box_count += 1;
                }
            }
            if box_count <= 0 {
                change_aegis(fighter,status,Type::NONE);
            }
        }
    }
}

pub fn install() {
    unsafe {
        LookupSymbol(
            &mut FIGHTER_MANAGER,
            "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
            .as_bytes()
            .as_ptr(),
        );
    }
    install_agent_frame!(eflame);
    install_status_scripts!(
        attack_main,
        attack_s3_main,
        attack_hi3_main,
        attack_lw3_main,
        attack_s4_main,
        attack_hi4_main,
        attack_lw4_main,
        attack_air_main,
        special_hi_jump_main,
        special_hi_loop_main,
        special_lw_out,
        special_s_main,
        special_n_attack_main,
        special_n_hold_main,
    );
}
