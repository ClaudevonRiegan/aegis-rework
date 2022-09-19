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
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static::lazy_static! {
    pub static ref LIST: Mutex<List> = Mutex::new(List::new());
}

pub struct Type;

impl Type {
    pub const NORMAL: i32 = 0;
    pub const AERIAL: i32 = 1;
    pub const SPECIAL: i32 = 2;
}

pub struct List {
    pub list: Vec<Change>,
}

pub struct Change {
    pub motion: u64,
    pub frame: f32,
    pub rate: f32,
    pub status: i32,
    pub change: bool,
    pub air_attack: i32,
}

impl Change {
    pub fn new(motion: u64, frame: f32, rate: f32, status: i32, change: bool, air_attack: i32) -> Change {
        Change {
            motion: motion,
            frame: frame,
            rate: rate,
            status: status,
            change: change,
            air_attack: air_attack,
        }
    }
}

impl List {
    pub fn new() -> List {
        List {
            list: Vec::new(),
        }
    }

    pub fn push_to_list(&mut self, change: Change) {
        self.list.push(change);
    }

    pub fn update_list(&mut self, change: Change, entry_id: usize) {
        self.list[entry_id] = change;
    }
}

extern "C" {
    #[link_name = "\u{1}_ZN3app29FighterElementLinkEventChange13new_l2c_tableEv"]
    pub fn FighterElementLinkEventChange__new_l2c_table() -> smash::lib::L2CValue;
}

pub unsafe fn element_special_lw_main(fighter: &mut L2CFighterCommon) {
    let mut event = FighterElementLinkEventChange__new_l2c_table();
    event["link_event_kind_"].assign(&L2CValue::new_int(0x1cd83c14e3u64));
    event["object_id_"].assign(&L2CValue::I32(*BATTLE_OBJECT_ID_INVALID));
    let callable: extern "C" fn() -> *mut smash::app::LinkEvent = std::mem::transmute(event["new_instance_lua_"].get_ptr());
    let link_event = callable();
    lua_bind::LinkEvent::load_from_l2c_table(link_event,&event);
    LinkModule::send_event_nodes_struct(fighter.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, link_event, 0);
    lua_bind::LinkEvent::store_l2c_table(link_event);
    let deleter: extern "C" fn(*mut smash::app::LinkEvent) = std::mem::transmute(*((*(link_event as *const u64) + 0x8) as *const u64));
    deleter(link_event);
    let callable: extern "C" fn() -> *mut smash::app::LinkEvent = std::mem::transmute(event["new_instance_lua_"].get_ptr());
    let link_event = callable();
    lua_bind::LinkEvent::load_from_l2c_table(link_event, &event);
    LinkModule::send_event_nodes_struct(fighter.module_accessor, *ITEM_LINK_NO_HAVE, link_event, 0);
    lua_bind::LinkEvent::store_l2c_table(link_event);
    let deleter: extern "C" fn(*mut smash::app::LinkEvent) = std::mem::transmute(*((*(link_event as *const u64) + 0x8) as *const u64));
    deleter(link_event);
    let entry_id = WorkModule::get_int(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
    let fighter_info = lua_bind::FighterManager::get_fighter_information(fighter_manager,FighterEntryID(entry_id));
    if lua_bind::FighterInformation::is_rabbit_cap(fighter_info) {
        ItemModule::eject_attach(fighter.module_accessor, ItemKind(*ITEM_KIND_USAGIHAT),  true, true);
    }
    if lua_bind::FighterInformation::is_reflector(fighter_info) {
        ItemModule::eject_attach(fighter.module_accessor, ItemKind(*ITEM_KIND_BADGE), true, true);
    }
    if lua_bind::FighterInformation::is_rocketbelt(fighter_info) {
        ItemModule::eject_attach(fighter.module_accessor, ItemKind(*ITEM_KIND_ROCKETBELT), true, true);
    }
    if lua_bind::FighterInformation::is_screw(fighter_info) {
        ItemModule::eject_attach(fighter.module_accessor, ItemKind(*ITEM_KIND_SCREW), true, true);
    }
    if lua_bind::FighterInformation::is_backshield(fighter_info) {
        ItemModule::eject_attach(fighter.module_accessor, ItemKind(*ITEM_KIND_BACKSHIELD), true, true);
    }
    AreaModule::set_whole(fighter.module_accessor,false);
    fighter.change_status(FIGHTER_ELEMENT_STATUS_KIND_SPECIAL_LW_STANDBY.into(),false.into());
}

pub unsafe fn change_aegis(fighter: &mut L2CFighterCommon, status: i32, kind: i32) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let motion = MotionModule::motion_kind(module_accessor);
    let frame = MotionModule::frame(module_accessor);
    let rate = MotionModule::rate(module_accessor);
    let attack_air = ControlModule::get_attack_air_kind(module_accessor);
    match kind {
        Type::NORMAL => {
            let change = Change::new(motion,frame,rate,status,true,-1);
            LIST.lock().unwrap().update_list(change,entry_id);
        },
        Type::AERIAL => {
            let change = Change::new(motion,frame,rate,status,true,attack_air);
            LIST.lock().unwrap().update_list(change,entry_id);
        },
        Type::SPECIAL => {
            let change = Change::new(0,0.0,0.0,status,false,-1);
            LIST.lock().unwrap().update_list(change,entry_id);
        }
        _ => {},
    };
    element_special_lw_main(fighter);
}

pub fn install() {
    lazy_static::initialize(&LIST);
    for _ in 0..8 {
        let change = Change::new(0,0.0,0.0,-1,false,-1);
        LIST.lock().unwrap().push_to_list(change);
    }
}
