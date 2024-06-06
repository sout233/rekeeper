use rea_rs::PluginContext;
use rea_rs_macros::reaper_extension_plugin;
use std::{collections::HashMap, error::Error, thread};

mod config;
mod core;
mod helper;

#[reaper_extension_plugin]
fn plugin_main(context: PluginContext) -> Result<(), Box<dyn Error>> {
    // 开始初始化咯咯咯
    match core::init(context) {
        Ok(_) => println!("Initialized"),
        Err(err) => helper::rpr_show_msg_box(&err.to_string()),
    }

    // 读读配置文件
    let new_config = match config::read_config() {
        Some(cfg) => cfg,
        None => {
            helper::rpr_cprintln("error when read config");
            // 暂且先搞个空的HashMap传回去吧
            let hsmp: HashMap<String, Option<u8>> = HashMap::new();
            hsmp
        }
    };

    // 开始监听键盘事件
    thread::spawn(|| {
        core::get_key(new_config);
    });

    // 就酱！
    Ok(())
}
