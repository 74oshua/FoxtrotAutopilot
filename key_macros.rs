use std::{env::var_os, fmt::Debug, io::BufReader, path::{Path, PathBuf}, thread::sleep, time::Duration};
use std::sync::{Arc, Mutex};

use crate::{journal::{GameState, JournalReader}, key_macros};
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use inputbot::{KeySequence, KeybdKey};
use inputbot::KeybdKey::*;
use std::sync::mpsc;

use std::fs::File;
use std::io::{self, BufRead, Error};
use glob::glob;
use xml::{attribute, reader::{EventReader, XmlEvent}};

struct Bindings
{
    pub up: KeybdKey,
    pub down: KeybdKey,
    pub menu_left: KeybdKey,
    pub right: KeybdKey,
    pub select: KeybdKey,
    pub back: KeybdKey,
    pub galmap: KeybdKey,
}

impl Default for Bindings
{
    fn default() -> Self
    {
        Bindings { up: WKey, down: SKey, menu_left: AKey, right: DKey, select: SpaceKey, back: BackspaceKey, galmap: MKey }
    }
}

impl Bindings
{
    pub fn str_to_key(string: String) -> Option<KeybdKey>
    {
        if string == "Key_A"
        {
            return Some(KeybdKey::AKey);
        }
        else if string == "Key_B"
        {
            return Some(KeybdKey::BKey);
        }
        else if string == "Key_C"
        {
            return Some(KeybdKey::CKey);
        }
        else if string == "Key_D"
        {
            return Some(KeybdKey::DKey);
        }
        else if string == "Key_E"
        {
            return Some(KeybdKey::EKey);
        }
        else if string == "Key_F"
        {
            return Some(KeybdKey::FKey);
        }
        else if string == "Key_G"
        {
            return Some(KeybdKey::GKey);
        }
        else if string == "Key_H"
        {
            return Some(KeybdKey::HKey);
        }
        else if string == "Key_I"
        {
            return Some(KeybdKey::IKey);
        }
        else if string == "Key_J"
        {
            return Some(KeybdKey::JKey);
        }
        else if string == "Key_K"
        {
            return Some(KeybdKey::KKey);
        }
        else if string == "Key_L"
        {
            return Some(KeybdKey::LKey);
        }
        else if string == "Key_M"
        {
            return Some(KeybdKey::MKey);
        }
        else if string == "Key_N"
        {
            return Some(KeybdKey::NKey);
        }
        else if string == "Key_O"
        {
            return Some(KeybdKey::OKey);
        }
        else if string == "Key_P"
        {
            return Some(KeybdKey::PKey);
        }
        else if string == "Key_Q"
        {
            return Some(KeybdKey::QKey);
        }
        else if string == "Key_R"
        {
            return Some(KeybdKey::RKey);
        }
        else if string == "Key_S"
        {
            return Some(KeybdKey::SKey);
        }
        else if string == "Key_T"
        {
            return Some(KeybdKey::TKey);
        }
        else if string == "Key_U"
        {
            return Some(KeybdKey::UKey);
        }
        else if string == "Key_V"
        {
            return Some(KeybdKey::VKey);
        }
        else if string == "Key_W"
        {
            return Some(KeybdKey::WKey);
        }
        else if string == "Key_X"
        {
            return Some(KeybdKey::XKey);
        }
        else if string == "Key_Y"
        {
            return Some(KeybdKey::YKey);
        }
        else if string == "Key_Z"
        {
            return Some(KeybdKey::ZKey);
        }
        // else if string == "Key_DownArrow"
        // {
        //     return Some(KeybdKey::DownKey);
        // }
        // else if string == "Key_UpArrow"
        // {
        //     return Some(KeybdKey::UpKey);
        // }
        // else if string == "Key_RightArrow"
        // {
        //     return Some(KeybdKey::RightKey);
        // }
        // else if string == "Key_LeftArrow"
        // {
        //     return Some(KeybdKey::LeftKey);
        // }
        else if string == "Key_0"
        {
            return Some(KeybdKey::Numrow0Key);
        }
        else if string == "Key_1"
        {
            return Some(KeybdKey::Numrow1Key);
        }
        else if string == "Key_2"
        {
            return Some(KeybdKey::Numrow2Key);
        }
        else if string == "Key_3"
        {
            return Some(KeybdKey::Numrow3Key);
        }
        else if string == "Key_4"
        {
            return Some(KeybdKey::Numrow4Key);
        }
        else if string == "Key_5"
        {
            return Some(KeybdKey::Numrow5Key);
        }
        else if string == "Key_6"
        {
            return Some(KeybdKey::Numrow6Key);
        }
        else if string == "Key_7"
        {
            return Some(KeybdKey::Numrow7Key);
        }
        else if string == "Key_8"
        {
            return Some(KeybdKey::Numrow8Key);
        }
        else if string == "Key_9"
        {
            return Some(KeybdKey::Numrow9Key);
        }
        else if string == "Key_Space"
        {
            return Some(KeybdKey::SpaceKey);
        }
        else if string == "Key_Backspace"
        {
            return Some(KeybdKey::BackspaceKey);
        }
        else if string == "Key_Enter"
        {
            return Some(KeybdKey::EnterKey);
        }
        else if string == "Key_LeftAlt"
        {
            return Some(KeybdKey::LAltKey);
        }
        else if string == "Key_RightAlt"
        {
            return Some(KeybdKey::RAltKey);
        }
        else if string == "Key_LeftControl"
        {
            return Some(KeybdKey::LControlKey);
        }
        else if string == "Key_RightControl"
        {
            return Some(KeybdKey::RControlKey);
        }
        else if string == "Key_Equals"
        {
            return Some(KeybdKey::EqualKey);
        }
        else if string == "Key_Tab"
        {
            return Some(KeybdKey::TabKey);
        }
        None
    }

    // get binding keys from file
    // from https://docs.rs/crate/xml-rs/latest
    fn from_file(path: PathBuf) -> Result<Bindings, io::Error>
    {
        let mut bindings = Bindings::default();
        let file = File::open(path)?;
        let file = BufReader::new(file);

        let parser = EventReader::new(file);
        let mut parents: Vec<String> = vec![];
        let mut depth = 0;
        for e in parser
        {
            match e
            {
                Ok(XmlEvent::StartElement { name, attributes, .. }) =>
                {
                    if parents.len() > 0
                    {
                        let parent = parents[parents.len()-1].clone();
                        if parent == "UI_Up"
                        {
                            for attribute in attributes
                            {
                                if attribute.name.to_string() == "Key"
                                {
                                    if let Some(key) = Bindings::str_to_key(attribute.value.to_string())
                                    {
                                        bindings.up = key;
                                    }
                                }
                            }
                        }
                        else if parent == "UI_Down"
                        {
                            for attribute in attributes
                            {
                                if attribute.name.to_string() == "Key"
                                {
                                    if let Some(key) = Bindings::str_to_key(attribute.value.to_string())
                                    {
                                        bindings.down = key;
                                    }
                                }
                            }
                        }
                        else if parent == "UI_Left"
                        {
                            for attribute in attributes
                            {
                                if attribute.name.to_string() == "Key"
                                {
                                    if let Some(key) = Bindings::str_to_key(attribute.value.to_string())
                                    {
                                        bindings.menu_left = key;
                                    }
                                }
                            }
                        }
                        else if parent == "UI_Right"
                        {
                            for attribute in attributes
                            {
                                if attribute.name.to_string() == "Key"
                                {
                                    if let Some(key) = Bindings::str_to_key(attribute.value.to_string())
                                    {
                                        bindings.right = key;
                                    }
                                }
                            }
                        }
                        else if parent == "UI_Select"
                        {
                            for attribute in attributes
                            {
                                if attribute.name.to_string() == "Key"
                                {
                                    if let Some(key) = Bindings::str_to_key(attribute.value.to_string())
                                    {
                                        bindings.select = key;
                                    }
                                }
                            }
                        }
                        else if parent == "UI_Back"
                        {
                            for attribute in attributes
                            {
                                if attribute.name.to_string() == "Key"
                                {
                                    if let Some(key) = Bindings::str_to_key(attribute.value.to_string())
                                    {
                                        bindings.back = key;
                                    }
                                }
                            }
                        }
                        else if parent == "GalaxyMapOpen"
                        {
                            for attribute in attributes
                            {
                                if attribute.name.to_string() == "Key"
                                {
                                    if let Some(key) = Bindings::str_to_key(attribute.value.to_string())
                                    {
                                        bindings.galmap = key;
                                    }
                                }
                            }
                        }
                    }

                    parents.push(name.to_string());
                    depth += 1;
                },
                Ok(XmlEvent::EndElement { name, .. }) =>
                {
                    parents.pop();
                    depth -= 1;
                },
                _ => ()
            }
        }

        Ok(bindings)
    }

    pub fn get_custom() -> Option<Bindings>
    {
        if let Some(appdata_path) = var_os("APPDATA").map(PathBuf::from)
        {
            if let Some(bindings_path) = appdata_path.parent()
            {
                let bindings_glob = bindings_path.join(PathBuf::from("Local\\Frontier Developments\\Elite Dangerous\\Options\\Bindings\\Custom.*.binds"));
                // get all custom bindings files
                if let Ok(custom_bindings) = glob(&bindings_glob.to_string_lossy().to_string()[..])
                {
                    // add the name of every custom bindings file to custom_bindings_list
                    let mut custom_bindings_list: Vec<String> = vec![];
                    for file in custom_bindings
                    {
                        match file
                        {
                            Ok(path) => custom_bindings_list.push(path.to_string_lossy().to_string()),
                            Err(_) => ()
                        }
                    }
                    // sort list
                    custom_bindings_list.sort();
                    // get newest bindings
                    if let Some(current_binding) = custom_bindings_list.last()
                    {
                        if let Ok(bindings) = Bindings::from_file(PathBuf::from(current_binding))
                        {
                            println!("Found bindings file: {}", current_binding);
                            return Some(bindings);
                        }
                    }
                }
            }
        }
        None
    }

    pub fn get_custom_or_default() -> Bindings
    {
        if let Some(bindings) = Bindings::get_custom()
        {
            return bindings;
        }
        Bindings::default()
    }
}

pub struct MacroChannels
{
    pub fc_route_cancel: Option<mpsc::Sender<bool>>,
}

impl Default for MacroChannels
{
    fn default() -> Self
    {
        MacroChannels { fc_route_cancel: None }
    }
}

    // pub fn new() -> KeyMacros
    // {
    //     if let Some(appdata_path) = var_os("APPDATA").map(PathBuf::from)
    //     {
    //         if let Some(bindings_path) = appdata_path.parent()
    //         {
    //             let bindings_glob = bindings_path.join(PathBuf::from("Local\\Frontier Developments\\Elite Dangerous\\Options\\Bindings\\Custom.*.binds"));
    //             // get all custom bindings files
    //             if let Ok(custom_bindings) = glob(&bindings_glob.to_string_lossy().to_string()[..])
    //             {
    //                 // add the name of every custom bindings file to custom_bindings_list
    //                 let mut custom_bindings_list: Vec<String> = vec![];
    //                 for file in custom_bindings
    //                 {
    //                     match file
    //                     {
    //                         Ok(path) => custom_bindings_list.push(path.to_string_lossy().to_string()),
    //                         Err(_) => ()
    //                     }
    //                 }
    //                 // sort list
    //                 custom_bindings_list.sort();
    //                 // get newest bindings
    //                 if let Some(current_binding) = custom_bindings_list.last()
    //                 {
    //                     if let Ok(bindings) = Bindings::from_file(PathBuf::from(current_binding))
    //                     {
    //                         println!("Using bindings in {}", current_binding);
    //                         return KeyMacros { bindings: bindings, macro_states: MacroStates::default() };
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     println!("Using default bindings");
    //     KeyMacros { bindings: Bindings::default(), macro_states: MacroStates::default() }
    // }

// key macros are pub static functions
fn sleep_ms(millis: u64)
{
    sleep(Duration::from_millis(millis));
}

fn press_key(key: inputbot::KeybdKey, times: usize)
{
    for _ in 0..times
    {
        key.press();
        sleep_ms(100);
        key.release();
        sleep_ms(100);
    }
}

fn hold_key(key: inputbot::KeybdKey, times: usize, duration: u64)
{
    for _ in 0..times
    {
        key.press();
        sleep_ms(duration);
        key.release();
        sleep_ms(100);
    }
}

// FC route macro
pub fn fc_auto_route(game_state_mutex: Arc<Mutex<GameState>>, gs_channel: mpsc::Receiver<bool>)
{
    // try to get custom bindings, otherwise use default
    let bindings = Bindings::get_custom_or_default();
    let mut last_game_state = GameState::new();

    // get clipboard context
    if let Ok(ctx) = ClipboardContext::new()
    {
        loop
        {
            // lock gamestate
            if let Ok(game_state) = game_state_mutex.lock()
            {
                last_game_state = game_state.clone();
            }

            // if in docking menu
            if last_game_state.music_track != "FleetCarrier_Managment" && last_game_state.music_track != "GalaxyMap"
            {
                press_key(bindings.back, 2);
                press_key(bindings.up, 3);
                press_key(bindings.down, 3);
                press_key(bindings.select, 1);

                // wait for carrier services
                sleep_ms(5000);

                press_key(bindings.down, 2);
                press_key(bindings.right, 2);
                press_key(bindings.select, 1);

                // wait for transition to carrier management
                while last_game_state.music_track != "FleetCarrier_Managment"
                {
                    // lock gamestate
                    if let Ok(game_state) = game_state_mutex.lock()
                    {
                        last_game_state = game_state.clone();
                    }
                    sleep_ms(100);
                }
                sleep_ms(5000);
            }

            // if in carrier management
            if last_game_state.music_track == "FleetCarrier_Managment"
            {
                press_key(bindings.up, 13);
                press_key(bindings.down, 1);
                press_key(bindings.select, 2);

                // wait for transition to map screen
                while last_game_state.music_track != "GalaxyMap"
                {
                    // lock gamestate
                    if let Ok(game_state) = game_state_mutex.lock()
                    {
                        last_game_state = game_state.clone();
                    }
                    sleep_ms(100);
                }
                sleep_ms(1000);
            }

            // if in map screen
            if last_game_state.music_track == "GalaxyMap"
            {
                
            }
        }
    }

    // loop
    // {
    //     // lock game and macro states
    //     if let Ok(game_state) = game_state_mutex.lock()
    //     {
    //         // return if macro cancelled
    //         if let Ok(should_cancel) = cancel_channel.recv()
    //         {
    //             if should_cancel
    //             {
    //                 return;
    //             }
    //         }
    //         // break loop if galmap event detected
    //         if game_state.music_track != "GalaxyMap"
    //         {
    //             break;
    //         }
    //     }
    //     sleep_ms(100);
    // }

    // sleep_ms(2000);
    // inputbot::MouseCursor::move_abs(1280, 150);

    // leave station
    // Self::press_key(self.bindings.menu_down, 1);
    // Self::press_key(self.bindings.menu_select, 1);
}