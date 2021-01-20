#![feature(proc_macro_hygiene)]
#![feature(asm)]
#![allow(unused_macros)]
#![allow(unused_unsafe)]

pub use status_proc::*;

use smash::*;

type StatusFunc = unsafe fn(l2c_fighter: *mut lua2cpp::L2CFighterCommon, l2c_agent: u64) -> lib::L2CValue;

#[macro_export]
macro_rules! replace_statuses {
    ($($func:ident),* $(,)?) => {
        $(
            $crate::replace_status!($func);
         )*
    };
}

// Defined in status_hook crate
extern "Rust" {
    pub fn replace_status_func(fighter_str: &'static str, status_kind: i32, lua_script: i32, func: StatusFunc);
    pub fn call_original(
        fighter_str: &'static str,
        status_kind: i32,
        lua_script: i32,
        l2c_fighter: *mut lua2cpp::L2CFighterCommon,
        l2c_agent: u64
    ) -> lib::L2CValue;
}
