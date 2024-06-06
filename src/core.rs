use device_query::{DeviceEvents, DeviceState};
use rea_rs::{PluginContext, Reaper};
use std::{collections::HashMap, env, error::Error, sync::Arc};

use super::helper;

static mut IS_ENABLE: bool = true;

pub(crate) fn init(context: PluginContext) -> Result<(), Box<dyn Error>> {
    Reaper::init_global(context);

    let message = "Hello from rekeeper";
    helper::rpr_cprintln(message);

    // 搞不懂目录了……不想搞多平台zzz
    let binding_run_path = env::current_dir().unwrap_or_default();
    let run_path = binding_run_path.as_os_str().to_str().unwrap_or_default();
    helper::rpr_cprintln(run_path);

    let reaper = Reaper::get_mut();

    // 注册action
    let _ = &reaper.register_action(
        "RLP_TE",
        "Rekeeper: Toggle Enable",
        |a: i32| {
            let _ = &helper::rpr_cprintln(&a.to_string());
            unsafe { IS_ENABLE = !IS_ENABLE }
            Ok(())
        },
        None,
    );

    Ok(())
}

pub(crate) fn get_key(cfg: HashMap<String, Option<u8>>) {
    let device_state = DeviceState::new();
    let cfg = Arc::new(cfg);

    // 搞不定所有权问题了，就先多clone几份吧
    let cfg_clone = Arc::clone(&cfg);

    // 键盘按下事件
    let _guard = device_state.on_key_down(move |key| {
        if unsafe { IS_ENABLE } {
            let reaper = Reaper::get();
            helper::rpr_cprintln(&format!("Keyboard key down: {:#?}", key));
            let reaper = reaper.low();
            let binding_cfg = &cfg_clone;
            let midi = binding_cfg
                .get(format!("{:?}", key).to_string().as_str())
                .unwrap_or(&None);
            
            match midi {
                Some(midi) => reaper.StuffMIDIMessage(0, 0x90, *midi as i32, 127),
                None => (),
            }
        }
    });

    let cfg_clone = Arc::clone(&cfg);

    // 键盘抬起事件
    let _guard = device_state.on_key_up(move |key| {
        if unsafe { IS_ENABLE } {
            let reaper = Reaper::get();
            helper::rpr_cprintln(&format!("Keyboard key up: {:#?}", key));
            let reaper = reaper.low();
            let binding_cfg = &cfg_clone;
            let midi = binding_cfg
                .get(format!("{:?}", key).to_string().as_str())
                .unwrap_or(&None);

            match midi {
                Some(midi) => reaper.StuffMIDIMessage(0, 0x80, *midi as i32, 0),
                None => (),
            }
        }
    });
    // 需要一直保持运行这个线程，才能监听到
    loop {}
}
