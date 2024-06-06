use device_query::{DeviceEvents, DeviceState};

mod config;

fn main() {
    let device_state = DeviceState::new();

    let _guard = device_state.on_key_down(|key| {
       println!("Keyboard key down: {:#?}", key);
    });
    let _guard = device_state.on_key_up(|key| {
       println!("Keyboard key up: {:#?}", key);
    });
   
    loop {}
    
    let cfg=config::read_config().unwrap();
    println!("{:?}",cfg)
}
