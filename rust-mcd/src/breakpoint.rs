//! Abstracts over breakpoints for a [crate::core::Core]
//!
//! TODO The implementation is very rudimentary and possibly wrong
use crate::mcd_bindings::{
    enum_mcd_trig_action_et, enum_mcd_trig_opt_et, enum_mcd_trig_type_et, mcd_addr_st,
    mcd_trig_simple_core_st,
};

pub enum TriggerType {
    RW,
    IP,
}

impl TriggerType {
    fn as_type(&self) -> enum_mcd_trig_type_et {
        match self {
            TriggerType::RW => enum_mcd_trig_type_et::MCD_TRIG_TYPE_RW,
            TriggerType::IP => enum_mcd_trig_type_et::MCD_TRIG_TYPE_IP,
        }
    }
}

impl mcd_trig_simple_core_st {
    pub(crate) fn create_trigger(trigger_type: TriggerType, address: u64, size: u64) -> Self {
        // For the Aurix Lite Kit v2 connected over micro-USB, MCD_TRIG_OPT_DEFAULT
        // as the option flag and MCD_TRIG_ACTION_DBG_DEBUG as the action flag are
        // the only supported flags (? or are we missing something here)
        Self {
            struct_size: core::mem::size_of::<mcd_trig_simple_core_st>() as u32,
            type_: trigger_type.as_type().0,
            option: enum_mcd_trig_opt_et::MCD_TRIG_OPT_DEFAULT.0,
            action: enum_mcd_trig_action_et::MCD_TRIG_ACTION_DBG_DEBUG.0,
            action_param: Default::default(),
            modified: Default::default(),
            state_mask: Default::default(),
            addr_start: mcd_addr_st {
                address,
                mem_space_id: 0,
                addr_space_id: 0,
                addr_space_type: 0,
            },
            addr_range: size,
        }
    }
}
