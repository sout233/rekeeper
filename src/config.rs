use directories::ProjectDirs;
use musical_note::ResolvedNote;
use rea_rs::Reaper;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Serialize, Deserialize)]
struct KeyMap {
    #[serde(flatten)]
    mapping: HashMap<String, String>,
}

fn piano_note_to_midi(note: &str) -> Option<u8> {
    let note = note.replace("#", "is");
    let note = note.to_lowercase();
    let midi = ResolvedNote::from_str(note.as_str())?;
    Some(midi.midi)
}

pub fn read_config() -> Option<HashMap<String, u8>> {
    let project_dirs = ProjectDirs::from("fm", "", "Reaper").expect("Failed to get project directories");

    let config_dir = project_dirs.config_dir();

    let binding = config_dir.join("key_mapping.json");
    let path = binding.to_str().unwrap_or_default();
    
    Reaper::get().show_console_msg(path);

    // 读取JSON文件
    let json_content = fs::read_to_string(path).ok()?;
    // 解析JSON内容
    let keyboard_to_piano: KeyMap = serde_json::from_str(&json_content).ok()?;

    let mut new_map: HashMap<String, u8> = HashMap::new();
    // 遍历映射，将钢琴音名称转换为MIDI数值
    for (kb_key, piano_note) in &keyboard_to_piano.mapping {
        let midi_value = piano_note_to_midi(&piano_note);
        new_map.insert(kb_key.to_string(), midi_value.unwrap_or(0));
    }
    Some(new_map)
}
