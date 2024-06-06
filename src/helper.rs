use rea_rs::Reaper;

pub fn rpr_cprintln(msg: &str) {
    let reaper = Reaper::get();
    #[cfg(debug_assertions)]
    reaper.show_console_msg(msg);
}

pub fn rpr_show_msg_box(msg: &str){
    let reaper = Reaper::get();
    let _ = reaper.show_message_box("Rekeeper", msg, rea_rs::MessageBoxType::Ok);
}