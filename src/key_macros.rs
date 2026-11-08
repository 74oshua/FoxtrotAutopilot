use std::{env::var_os, io::BufReader, path::{PathBuf}, thread::sleep, time::Duration};
use std::sync::{Arc, Mutex};
use active_win_pos_rs::*;

use crate::GameState;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use inputbot::{KeybdKey};
use inputbot::KeybdKey::*;
use std::sync::mpsc;

use std::fs::File;
use std::io::{self};
use glob::glob;
use xml::{reader::{EventReader, XmlEvent}};

struct Bindings
{
    pub up: KeybdKey,
    pub down: KeybdKey,
    pub left: KeybdKey,
    pub right: KeybdKey,
    pub select: KeybdKey,
    pub back: KeybdKey,
    pub galmap: KeybdKey,
}

impl Default for Bindings
{
    fn default() -> Self
    {
        Bindings { up: WKey, down: SKey, left: AKey, right: DKey, select: SpaceKey, back: BackspaceKey, galmap: MKey }
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
                                        bindings.left = key;
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
                },
                Ok(XmlEvent::EndElement { .. }) =>
                {
                    parents.pop();
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

fn get_resolution_from_settings(path: PathBuf) -> Result<WindowPosition, io::Error>
{
    let file = File::open(path)?;
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut parents: Vec<String> = vec![];
    let mut screen_width: Option<u32> = None;
    let mut screen_height: Option<u32> = None;
    for e in parser
    {
        match e
        {
            Ok(XmlEvent::StartElement { name, .. }) =>
            {
                parents.push(name.to_string());
            },
            Ok(XmlEvent::Characters { 0: string }) =>
            {
                if parents.len() > 0
                {
                    let parent = parents[parents.len()-1].clone();

                    // if not windowed, go with window coordinates
                    if parent == "Fullscreen"
                    {
                        if string != "0".to_string()
                        {
                            if let Ok(window_pos) = get_position()
                            {
                                return Ok(window_pos);
                            }
                        }
                    }
                    else if parent == "ScreenWidth"
                    {
                        if let Ok(width) = u32::from_str_radix(&string, 10)
                        {
                            screen_width = Some(width);
                        }
                    }
                    else if parent == "ScreenHeight"
                    {
                        if let Ok(height) = u32::from_str_radix(&string, 10)
                        {
                            screen_height = Some(height);
                        }
                    }
                }
            },
            Ok(XmlEvent::EndElement { .. }) =>
            {
                parents.pop();
            },
            _ => ()
        }
    }
    if let Some(width) = screen_width
    {
        if let Some(height) = screen_height
        {
            if let Ok(window_pos) = get_position()
            {
                let height_difference = window_pos.height - height as f64;
                return Ok(WindowPosition { x: window_pos.x, y: window_pos.y + height_difference, width: width as f64, height: height as f64 });
            }
        }
    }
    Err(io::Error::new(io::ErrorKind::InvalidData, "Could not find resolution in display settings file"))
}

fn get_game_position() -> Result<WindowPosition, io::Error>
{
    if let Some(appdata_path) = var_os("APPDATA").map(PathBuf::from)
    {
        if let Some(display_settings_path) = appdata_path.parent()
        {
            let display_settings = display_settings_path.join(PathBuf::from("Local\\Frontier Developments\\Elite Dangerous\\Options\\Graphics\\DisplaySettings.xml"));
            return get_resolution_from_settings(display_settings);
        }
        else
        {
            println!("Could not find DisplaySettings.xml");
        }
    }
    else
    {
        println!("Could not find DisplaySettings.xml");
    }
    Err(io::Error::new(io::ErrorKind::InvalidData, "x"))
}

fn get_window_position() -> Result<WindowPosition, io::Error>
{
    if let Ok(position) = get_game_position()
    {
        Ok(position)
    }
    else if let Ok(position) = get_position()
    {
        Ok(position)
    }
    else
    {
        Err(io::Error::new(io::ErrorKind::InvalidData, "x"))
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

fn check_elite_focused()
{
    // pause if Elite is not focused
    let mut window_name = "".to_string();
    if let Ok(window) = get_active_window()
    {
        window_name = window.app_name;
    }
    if window_name != "Elite Dangerous: Odyssey Executable"
    {
        println!("Waiting for Elite Dangerous to be focused...");
        while window_name != "Elite Dangerous: Odyssey Executable"
        {
            if let Ok(window) = get_active_window()
            {
                window_name = window.app_name;
            }
            sleep_ms(100);
        }
        println!("Elite Dangerous focused");
    }
}

// key macros are pub static functions
fn sleep_ms(millis: u64)
{
    sleep(Duration::from_millis(millis));
}

fn press_key(key: inputbot::KeybdKey, times: usize)
{
    check_elite_focused();
    for _ in 0..times
    {
        key.press();
        sleep_ms(100);
        key.release();
        sleep_ms(100);
    }
}

// x and y are proportional to the active window
// (1, 1) is the bottom right corner
fn left_click(x: f64, y: f64)
{
    check_elite_focused();
    // translate coordinates to window
    if let Ok(position) = get_window_position()
    {
        // println!("({}, {}), {}x{}", position.x, position.y, position.width, position.height);
        inputbot::MouseCursor::move_abs(
            (position.x + x * position.width) as i32,
            (position.y + y * position.height) as i32);
        inputbot::MouseButton::LeftButton.press();
        sleep_ms(100);
        check_elite_focused();
        inputbot::MouseButton::LeftButton.release();
        sleep_ms(100);
        check_elite_focused();
    }
}

fn hold_key(key: inputbot::KeybdKey, times: usize, duration: u64)
{
    check_elite_focused();
    for _ in 0..times
    {
        key.press();
        sleep_ms(duration);
        key.release();
        sleep_ms(100);
    }
}

fn check_journal(game_state_mutex: &Arc<Mutex<GameState>>, cancel_channel: &mpsc::Receiver<bool>) -> Option<GameState>
{
    check_elite_focused();

    // return if cancelled
    if cancel_channel.try_recv().is_ok()
    {
        None
    }
    // lock gamestate
    else if let Ok(game_state) = game_state_mutex.lock()
    {
        Some(game_state.clone())
    }
    else
    {
        None
    }
}

// FC route macro
pub fn fc_auto_route(game_state_mutex: &Arc<Mutex<GameState>>, cancel_channel: mpsc::Receiver<bool>)
{
    // try to get custom bindings, otherwise use default
    let bindings = Bindings::get_custom_or_default();
    let mut last_game_state = GameState::new();

    // get clipboard context
    if let Ok(mut ctx) = ClipboardContext::new()
    {
        loop
        {
            // lock gamestate
            if let Ok(game_state) = game_state_mutex.lock()
            {
                last_game_state = game_state.clone();
            }

            // if not docked with carrier, wait until we are
            if !last_game_state.docked || (last_game_state.docked_station_type != Some("FleetCarrier".to_string()) && last_game_state.docked_station_type != Some("FleetCarrier".to_string()))
            {
                println!("Not docked with carrier, waiting for docking...");
                while !last_game_state.docked || (last_game_state.docked_station_type != Some("FleetCarrier".to_string()) && last_game_state.docked_station_type != Some("SquadronCarrier".to_string()))
                {
                    if let Some(game_state) = check_journal(&game_state_mutex, &cancel_channel)
                    {
                        last_game_state = game_state;
                    }
                    else
                    {
                        return;
                    }
                    sleep_ms(100);
                }
                println!("Docking detected, beginning route...");
                sleep_ms(10000);
            }

            // if in docking menu
            if last_game_state.music_track != "FleetCarrier_Managment" && last_game_state.music_track != "GalaxyMap"
            {
                // println!("In docking menu");
                press_key(bindings.back, 2);
                press_key(bindings.up, 3);
                press_key(bindings.down, 1);
                press_key(bindings.select, 1);

                // wait for carrier services
                sleep_ms(5000);

                press_key(bindings.down, 2);
                press_key(bindings.right, 2);
                press_key(bindings.select, 1);

                // wait for transition to carrier management
                {
                    let mut timeout = 0;
                    while last_game_state.music_track != "FleetCarrier_Managment" && timeout < 100
                    {
                        if let Some(game_state) = check_journal(&game_state_mutex, &cancel_channel)
                        {
                            last_game_state = game_state;
                        }
                        else
                        {
                            return;
                        }
                        sleep_ms(100);
                        timeout += 1;
                    }
                    sleep_ms(5000);
                }
            }

            // if in carrier management
            if last_game_state.music_track == "FleetCarrier_Managment"
            {
                // println!("In carrier management");
                press_key(bindings.up, 13);
                press_key(bindings.down, 1);
                press_key(bindings.select, 2);

                // wait for transition to map screen
                let mut timeout = 0;
                while last_game_state.music_track != "GalaxyMap" && timeout < 100
                {
                    if let Some(game_state) = check_journal(&game_state_mutex, &cancel_channel)
                    {
                        last_game_state = game_state;
                    }
                    else
                    {
                        return;
                    }
                    sleep_ms(100);
                    timeout += 1;
                }
            }

            // if in map screen
            if let Some(docked_carrier_id) = last_game_state.docked_carrier_id
            {
                sleep_ms(1000);
                if last_game_state.music_track == "GalaxyMap" && last_game_state.docked_carrier_id.is_some()
                {
                    // println!("In galaxy map");
                    // get clipboard contents
                    if let Ok(clipboard) = ctx.get_contents()
                    {
                        let mut target = "".to_string();
                        if let Some(docked_carrier) = last_game_state.carriers.get(&docked_carrier_id)
                        {
                            // end of route
                            if docked_carrier.star_system == clipboard
                            {
                                println!("End of route");
                                return;
                            }
                            if let Some(target_system) = &docked_carrier.target_system
                            {
                                target = target_system.clone();
                            }
                        }
                        while target != clipboard
                        {
                            // if we are not already jumping to target, plot a course
                            println!("Plotting a course for {}...", clipboard);

                            // click system search bar
                            // left_click(pos_x + res_x / 2, pos_y + res_y / 8);
                            left_click(0.5, 0.125);
                            // left_click(0.5, 0.125);
                            // paste system name
                            LControlKey.press();
                            press_key(VKey, 1);
                            LControlKey.release();
                            // wait for results
                            sleep_ms(1000);
                            // click on first result
                            // left_click(res_x / 2, res_y / 7);
                            left_click(0.5, 0.143);
                            // left_click(0.5, 0.143);
                            // left_click(res_x / 4 * 3, res_y / 8 * 3);
                            left_click(0.75, 0.375);

                            // wait for carrier jump request
                            let mut timeout = 0;
                            while target != clipboard && timeout < 20
                            {
                                if let Some(game_state) = check_journal(&game_state_mutex, &cancel_channel)
                                {
                                    last_game_state = game_state;
                                }
                                else
                                {
                                    return;
                                }
                                sleep_ms(100);
                                // if the jump failed and we're back in Management...
                                if last_game_state.music_track == "FleetCarrier_Managment"
                                {
                                    // return to map
                                    sleep_ms(1000);
                                    press_key(bindings.down, 1);
                                    press_key(bindings.select, 1);
                                    sleep_ms(1000);
                                    press_key(bindings.select, 1);
                                    sleep_ms(1000);
                                }
                                // check new carrier target
                                if let Some(docked_carrier) = last_game_state.carriers.get(&docked_carrier_id)
                                {
                                    if let Some(ref docked_carrier_target) = docked_carrier.target_system
                                    {
                                        target = docked_carrier_target.clone();
                                    }
                                }
                                timeout += 1;
                            }
                        }
                        sleep_ms(3000);
                    }
                }
                // return to docking menu
                press_key(bindings.back, 1);
                sleep_ms(1000);
                press_key(bindings.back, 1);
                sleep_ms(1000);
                press_key(bindings.back, 1);
                sleep_ms(1000);
                press_key(bindings.select, 1);
            }

            // if in docking menu after carrier jump requested
            if last_game_state.music_track != "FleetCarrier_Managment" && last_game_state.music_track != "GalaxyMap"
            {
                // load tritium until fuel is above 300
                println!("Loading tritium");
                let mut low_fuel = true;
                let mut timeout = 0;
                while low_fuel && timeout < 5
                {
                    if let Some(game_state) = check_journal(&game_state_mutex, &cancel_channel)
                    {
                        last_game_state = game_state;
                    }
                    else
                    {
                        return;
                    }
                    sleep_ms(1000);
                
                    if let Some(docked_carrier_id) = last_game_state.docked_carrier_id
                    {
                        if let Some(docked_carrier) = last_game_state.carriers.get(&docked_carrier_id)
                        {
                            low_fuel = docked_carrier.fuel_level < 300;
                        }
                        // open commodities market
                        press_key(bindings.right, 2);
                        press_key(bindings.select, 1);
                        sleep_ms(1000);
                        
                        // get position of tritium on list
                        if let Some(market) = last_game_state.market
                        {
                            if let Some(tritium_position) = market.items.iter().position(|item| item.name == "$tritium_name;")
                            {
                                // move over to commodities list
                                press_key(bindings.right, 1);

                                // navigate down to tritium on the list
                                press_key(bindings.down, tritium_position);

                                press_key(bindings.select, 1);
                                hold_key(bindings.right, 1, 3000);
                                press_key(bindings.down, 1);
                                press_key(bindings.select, 1);
                                sleep_ms(1000);
                                press_key(bindings.back, 1);
                                sleep_ms(1000);
                                press_key(bindings.back, 1);
                                sleep_ms(1000);
                                press_key(bindings.back, 1);
                                sleep_ms(1000);

                                // load tritium in carrier
                                press_key(bindings.select, 1);
                                sleep_ms(5000);
                                press_key(bindings.down, 2);
                                press_key(bindings.select, 1);
                                sleep_ms(1000);
                                press_key(bindings.select, 1);
                                sleep_ms(1000);
                                press_key(bindings.up, 1);
                                press_key(bindings.select, 1);
                                sleep_ms(3000);
                            }
                        }


                        // exit to docking menu
                        press_key(bindings.back, 1);
                        sleep_ms(1000);
                        press_key(bindings.back, 1);
                        sleep_ms(1000);
                        press_key(bindings.back, 1);
                        sleep_ms(1000);
                        press_key(bindings.back, 1);
                        sleep_ms(1000);
                    }
                    timeout += 1;
                }
                
                // wait for jump to finish
                println!("Waiting for jump...");
                let mut carrier_arrived = false;
                while !carrier_arrived
                {
                    if let Some(game_state) = check_journal(&game_state_mutex, &cancel_channel)
                    {
                        last_game_state = game_state;
                    }
                    else
                    {
                        return;
                    }
                    sleep_ms(1000);
                    if let Some(docked_carrier_id) = last_game_state.docked_carrier_id
                    {
                        if let Some(docked_carrier) = last_game_state.carriers.get(&docked_carrier_id)
                        {
                            if let Some(ref target_system) = docked_carrier.target_system
                            {
                                carrier_arrived = target_system == &docked_carrier.star_system;
                            }
                        }
                    }
                }

                // wait 3 minutes for carrier navigation to unlock
                println!("Waiting for cooldown");
                for _ in 0..300
                {
                    if let Some(game_state) = check_journal(&game_state_mutex, &cancel_channel)
                    {
                        last_game_state = game_state;
                    }
                    else
                    {
                        return;
                    }
                    sleep_ms(1000);
                }
            }
        }
    }
    else
    {
        println!("Cannot get clipboard context");
    }
}

pub fn auto_trade(game_state_mutex: &Arc<Mutex<GameState>>, cancel_channel: mpsc::Receiver<bool>)
{
    // get gamestate
    let mut last_game_state = GameState::new();
    if let Ok(game_state) = game_state_mutex.lock()
    {
        last_game_state = game_state.clone();
    }

    loop
    {
        // if not docked, wait until we are
        if !last_game_state.docked
        {
            println!("Not docked, waiting for docking...");
            while !last_game_state.docked
            {
                if let Some(game_state) = check_journal(&game_state_mutex, &cancel_channel)
                {
                    last_game_state = game_state;
                }
                else
                {
                    return;
                }
                sleep_ms(100);
            }
            println!("Docking detected, beginning route...");
            sleep_ms(10000);
        }
    }
}