use std::{env::var_os, fmt::Debug, io::BufReader, path::{Path, PathBuf}, thread::sleep, time::Duration};

use crate::{journal::{GameState, JournalReader}, key_macros};
use inputbot::{KeySequence, KeybdKey};
use inputbot::KeybdKey::*;
use winit::event::Event;

use std::fs::File;
use std::io::{self, BufRead, Error};
use glob::glob;
use xml::{attribute, reader::{EventReader, XmlEvent}};

struct Bindings
{
    pub menu_up: KeybdKey,
    pub menu_down: KeybdKey,
    pub menu_left: KeybdKey,
    pub menu_right: KeybdKey,
    pub menu_select: KeybdKey,
    pub menu_back: KeybdKey,
    pub galmap: KeybdKey,
}

impl Default for Bindings
{
    fn default() -> Self
    {
        Bindings { menu_up: WKey, menu_down: SKey, menu_left: AKey, menu_right: DKey, menu_select: SpaceKey, menu_back: BackspaceKey, galmap: MKey }
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
                                        bindings.menu_up = key;
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
                                        bindings.menu_down = key;
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
                                        bindings.menu_right = key;
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
                                        bindings.menu_select = key;
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
                                        bindings.menu_back = key;
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
}

pub struct KeyMacros
{
    bindings: Bindings
}

impl Default for KeyMacros
{
    fn default() -> Self
    {
        KeyMacros { bindings: Bindings::default() }
    }
}

impl KeyMacros
{
    pub fn new() -> Self
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
                            println!("Using bindings in {}", current_binding);
                            return KeyMacros { bindings: bindings };
                        }
                    }
                }
            }
        }
        println!("Using default bindings");
        KeyMacros { bindings: Bindings::default() }
    }

    fn sleep_ms(millis: u64)
    {
        sleep(Duration::from_millis(millis));
    }

    fn press_key(key: inputbot::KeybdKey, times: usize)
    {
        for _ in 0..times
        {
            key.press();
            Self::sleep_ms(100);
            key.release();
            Self::sleep_ms(100);
        }
    }

    fn hold_key(key: inputbot::KeybdKey, times: usize, duration: u64)
    {
        for _ in 0..times
        {
            key.press();
            Self::sleep_ms(duration);
            key.release();
            Self::sleep_ms(100);
        }
    }

    // FC mover macro
    // move FC along clipboard route
    pub fn fc_mover_macro(game_state: &mut GameState, journal_reader: &mut JournalReader)
    {
        // if we are in the docking menu
        // (yes, FDev really did misspell management)
        if game_state.music_track != "FleetCarrier_Managment" && game_state.music_track != "GalaxyMap"
        {
            // check for journal updates
            journal_reader.update_journal(game_state);
        }
    }

    // automatic trade route
    // start at the buy system
    pub fn auto_trade(&self, game_state: &mut GameState, journal_reader: &mut JournalReader, commodity_list_pos: usize, buy_system: String, sell_system: String)
    {
        // // reset to top of docking menu
        // Self::press_key(self.bindings.menu_back, 3);
        // Self::press_key(self.bindings.menu_up, 3);

        // // open starport services
        // Self::press_key(self.bindings.menu_down, 1);
        // Self::press_key(self.bindings.menu_select, 1);
        // Self::sleep_ms(1000);

        // // open commodities
        // Self::press_key(self.bindings.menu_down, 1);
        // Self::press_key(self.bindings.menu_select, 1);

        // // wait for market event
        // while journal_reader.last_event != "Market"
        // {
        //     Self::sleep_ms(100);
        //     journal_reader.update_journal(game_state);
        // }
        // Self::sleep_ms(2000);

        // // buy as much of the target commodity as possible
        // Self::press_key(self.bindings.menu_right, 1);
        // Self::press_key(self.bindings.menu_down, commodity_list_pos-1);
        // Self::press_key(self.bindings.menu_select, 1);
        // Self::sleep_ms(1000);
        // Self::hold_key(self.bindings.menu_right, 1, 2000);
        // Self::press_key(self.bindings.menu_down, 1);
        // Self::press_key(self.bindings.menu_select, 1);

        // // exit to docking menu
        // Self::press_key(self.bindings.menu_back, 2);
        // Self::sleep_ms(500);
        // Self::press_key(self.bindings.menu_back, 1);

        // plot course to sell system
        Self::press_key(self.bindings.galmap, 1);
        // wait for galmap event
        while game_state.music_track != "GalaxyMap"
        {
            Self::sleep_ms(100);
            journal_reader.update_journal(game_state);
        }
        Self::sleep_ms(2000);
        inputbot::MouseCursor::move_abs(1280, 150);

        // leave station
        // Self::press_key(self.bindings.menu_down, 1);
        // Self::press_key(self.bindings.menu_select, 1);
    }
}