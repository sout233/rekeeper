use rea_rs::PluginContext;
use rea_rs_macros::reaper_extension_plugin;
use std::{collections::HashMap, error::Error, thread};

mod config;
mod core;
mod helper;

#[reaper_extension_plugin]
fn plugin_main(context: PluginContext) -> Result<(), Box<dyn Error>> {
    match core::init(context) {
        Ok(_) => println!("Initialized"),
        Err(err) => helper::rpr_show_msg_box(&err.to_string()),
    }

    let new_config = match config::read_config() {
        Some(cfg) => cfg,
        None => {
            helper::rpr_cprintln("error when read config");

            let hsmp: HashMap<String, u8> = HashMap::new();
            hsmp
        }
    };

    thread::spawn(|| {
        core::get_key(new_config);
    });

    Ok(())
}
