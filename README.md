# skyline-status
Status script replacements for Smash Ultimate

Note: You **\*need\*** to install the latest build of `libnro_hook.nro` from [here](https://github.com/ultimate-research/nro-hook-plugin/releases/tag/master).

If you don't, your game _will_ crash on boot!  You have been warned!

## Example Usage
```rust
#![feature(proc_macro_hygiene)]

use skyline_status::*;

use skyline::nro::{self, NroInfo};

use smash::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;

#[status_script("marth", FIGHTER_STATUS_KIND_SPECIAL_HI, LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn marth_turnaround(boma: &mut app::BattleObjectModuleAccessor) -> lib::L2CValue {
    original!();
    if MotionModule::frame(boma) == 4.0 {
        if ControlModule::get_stick_x(boma) * PostureModule::lr(boma) < 0.0 {
            PostureModule::reverse_lr(boma);
            PostureModule::update_rot_y_lr(boma);
        }
    }
    lib::L2CValue::new_int(0)
}

fn nro_main(nro: &NroInfo) {
    match nro.name {
        "common" => {
            replace_statuses!(marth_turnaround); // comma separate for more!
        }
        _ => ()
    }
}

#[skyline::main(name = "test_status")]
pub fn main() {
    println!("Hello from test plugin!");
    nro::add_hook(nro_main).unwrap();
}
```

## Example Cargo.toml
```
...
[package.metadata.skyline]
titleid = "01006A800016E000"
plugin-dependencies = [
    { name = "libnro_hook.nro", url = "https://github.com/ultimate-research/nro-hook-plugin/releases/download/master/libnro_hook.nro" },
    { name = "libstatus_hook.nro", url = "https://github.com/PeterHo8888/status_hook/releases/download/master/libstatus_hook.nro" },
]
...
[dependencies]
skyline = { git = "https://github.com/ultimate-research/skyline-rs.git" }
skyline_smash = { git = "https://github.com/ultimate-research/skyline-smash.git" }
skyline_status = { git = "https://github.com/PeterHo8888/skyline-status.git" }
...
```
