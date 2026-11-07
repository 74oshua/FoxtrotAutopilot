use std::sync::{Arc, Mutex};

use std::fs::File;
use std::io::{self, BufRead, Error};
use glob::glob;
use serde::Deserialize;

use crate::{GameState, Carrier, CarrierType};

// Elite Dangerous journal entry event struct
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EDEvent
{
    timestamp: String,
    event: String,
    #[serde(rename(deserialize = "Name"))]
    name: Option<String>,
    #[serde(rename(deserialize = "JumpType"))]
    jump_type: Option<String>,
    #[serde(rename(deserialize = "MusicTrack"))]
    music_track: Option<String>,
    #[serde(rename(deserialize = "CarrierID"))]
    carrier_id: Option<u32>,
    #[serde(rename(deserialize = "CarrierType"))]
    carrier_type: Option<String>,
    #[serde(rename(deserialize = "Callsign"))]
    callsign: Option<String>,
    #[serde(rename(deserialize = "StarSystem"))]
    star_system: Option<String>,
    #[serde(rename(deserialize = "FuelLevel"))]
    fuel_level: Option<u32>,
    #[serde(rename(deserialize = "SystemName"))]
    system_name: Option<String>,
    #[serde(rename(deserialize = "Docked"))]
    docked: Option<bool>,
    #[serde(rename(deserialize = "StationName"))]
    station_name: Option<String>,
    #[serde(rename(deserialize = "StationType"))]
    station_type: Option<String>,
}

pub struct JournalReader
{
    journal_iter: Option<io::Lines<io::BufReader<File>>>,
}

impl Default for JournalReader
{
    fn default() -> Self
    {
        JournalReader { journal_iter: None }
    }
}

impl JournalReader
{
    // based on https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
    // creates a new journal reader, creates a line iterator for newest journal in home directory
    pub fn new() -> io::Result<JournalReader>
    {
        // get home directory from $USERPROFILE
        if let Some(home_path) = std::env::home_dir()
        {
            // convert to string
            if let Some(home_str) = home_path.to_str()
            {
                // append journal directory to $USERPROFILE to get full journal path
                let mut journal_dir: String = home_str.to_owned();
                journal_dir.push_str("\\Saved Games\\Frontier Developments\\Elite Dangerous\\Journal.*-*-*.*.log");
                // get a list of every journal file in the journal directory
                if let Ok(journal_files) = glob(&journal_dir[..])
                {
                    // add the name of every journal file to journal_file_list
                    let mut journal_file_list: Vec<String> = vec![];
                    for file in journal_files
                    {
                        match file
                        {
                            Ok(path) => journal_file_list.push(path.to_string_lossy().to_string()),
                            Err(e) => return io::Result::Err(Error::new(io::ErrorKind::InvalidData, e))
                        }
                    }
                    // sort journal files by name
                    journal_file_list.sort();
                    // get newest
                    let current_journal_name = &journal_file_list[journal_file_list.len()-1];

                    // open newest journal file and return iterator
                    println!("Found journal file: {}", current_journal_name);
                    let file = File::open(current_journal_name)?;
                    Ok(JournalReader { journal_iter: Some(io::BufReader::new(file).lines()) })
                }
                else
                {
                    io::Result::Err(Error::new(io::ErrorKind::InvalidData, "glob error: creating glob"))
                }
            }
            else
            {
                io::Result::Err(Error::new(io::ErrorKind::InvalidData, "$USERPROFILE could not be converted to a string"))
            }
        }
        else
        {
            io::Result::Err(Error::new(io::ErrorKind::InvalidData, "$USERPROFILE not defined"))
        }
    }

    pub fn update_journal(&mut self, game_state_mut: &Arc<Mutex<GameState>>) -> Result<(), Error>
    {
        if let Ok(mut game_state) = game_state_mut.lock()
        {
            // try to get the next line from the journal
            if let Some(ref mut iter) = self.journal_iter
            {
                if let Some(next_line) = iter.next()
                {
                    // get the line as an EDEvent
                    if let Ok(event) = serde_json::from_str::<EDEvent>(&next_line?[..])
                    {
                        // save last event
                        game_state.last_event = event.event.clone();

                        // set FSD target
                        match event.event.as_str()
                        {
                            "FSDTarget" =>
                            {
                                if let Some(fsd_target) = event.name
                                {
                                    game_state.fsd_target = Some(fsd_target);
                                }
                                else
                                {
                                    println!("Error getting FSD target");
                                }
                            },
                            "Music" =>
                            {
                                if let Some(music_track) = event.music_track
                                {
                                    match music_track.as_str()
                                    {
                                        "DestinationFromHyperspace" => 
                                        {
                                            game_state.fsd_target = None;
                                        },
                                        _ => ()
                                    }
                                    game_state.music_track = music_track;
                                }
                                else
                                {
                                    println!("Error getting music track");
                                }
                            },
                            "CarrierStats" | "CarrierLocation" | "CarrierJumpRequest" =>
                            {
                                if let Some(carrier_id) = event.carrier_id
                                {
                                    // update jump destination
                                    if let Some(system_name) = event.system_name
                                    {
                                        game_state.carriers.entry(carrier_id).or_insert(Carrier::default()).target_system = Some(system_name);
                                    }
                                    if let Some(name) = event.name
                                    {
                                        game_state.carriers.entry(carrier_id).or_insert(Carrier::default()).name = name;
                                    }
                                    if let Some(carrier_type) = event.carrier_type
                                    {
                                        let ctype = match &carrier_type[..]
                                        {
                                            "FleetCarrier" => CarrierType::FleetCarrier,
                                            "SquadronCarrier" => CarrierType::SquadronCarrier,
                                            _ => CarrierType::Unknown
                                        };
                                        game_state.carriers.entry(carrier_id).or_insert(Carrier::default()).carrier_type = ctype;
                                    }
                                    if let Some(callsign) = event.callsign
                                    {
                                        game_state.carriers.entry(carrier_id).or_insert(Carrier::default()).callsign = callsign;
                                    }
                                    if let Some(star_system) = event.star_system
                                    {
                                        game_state.carriers.entry(carrier_id).or_insert(Carrier::default()).star_system = star_system;
                                    }
                                    if let Some(fuel_level) = event.fuel_level
                                    {
                                        game_state.carriers.entry(carrier_id).or_insert(Carrier::default()).fuel_level = fuel_level;
                                    }

                                    if (game_state.docked_station_type == Some("FleetCarrier".to_string()) || game_state.docked_station_type == Some("SquadronCarrier".to_string())) && game_state.docked_carrier_id.is_none()
                                    {
                                        if let Some(station_name) = &game_state.station_name
                                        {
                                            for carrier in &game_state.carriers
                                            {
                                                if carrier.1.callsign == *station_name
                                                {
                                                    game_state.docked_carrier_id = Some(*carrier.0);
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                            "CarrierJump" =>
                            {
                                if let Some(carrier_id) = event.carrier_id
                                {
                                    game_state.carriers.entry(carrier_id).or_insert(Carrier::default()).target_system = None;
                                }
                            }
                            "CarrierJumpCancelled" =>
                            {
                                if let Some(carrier_id) = event.carrier_id
                                {
                                    game_state.carriers.entry(carrier_id).or_insert(Carrier::default()).target_system = None;
                                }
                            },
                            "Location" =>
                            {
                                if let Some(docked) = event.docked
                                {
                                    game_state.docked = docked;
                                    if game_state.docked
                                    {
                                        if let Some(station_name) = event.station_name
                                        {
                                            if let Some(station_type) = event.station_type
                                            {
                                                if station_type == "FleetCarrier" || station_type == "SquadronCarrier"
                                                {
                                                    for carrier in &game_state.carriers
                                                    {
                                                        if carrier.1.callsign == station_name
                                                        {
                                                            game_state.docked_carrier_id = Some(*carrier.0);
                                                            break;
                                                        }
                                                    }
                                                }
                                                game_state.docked_station_type = Some(station_type);
                                            }
                                            game_state.station_name = Some(station_name);
                                        }
                                        else
                                        {
                                            game_state.station_name = None;
                                            game_state.docked_carrier_id = None;
                                        }
                                    }
                                }
                            },
                            "Docked" =>
                            {
                                game_state.docked = true;
                                if let Some(station_name) = event.station_name
                                {
                                    if let Some(station_type) = event.station_type
                                    {
                                        game_state.docked_station_type = Some(station_type.clone());
                                        if station_type == "FleetCarrier" || station_type == "SquadronCarrier"
                                        {
                                            for carrier in &game_state.carriers
                                            {
                                                if carrier.1.callsign == station_name
                                                {
                                                    game_state.docked_carrier_id = Some(*carrier.0);
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                    game_state.station_name = Some(station_name);
                                }
                            },
                            "Undocked" =>
                            {
                                game_state.docked = false;
                                game_state.docked_carrier_id = None;
                                game_state.station_name = None;
                                game_state.docked_station_type = None;
                            },
                            _ => ()
                        }
                    }
                }
                else
                {
                    game_state.current = true;
                }
                Ok(())
            }
            else
            {
                Err(Error::new(io::ErrorKind::InvalidData, "Journal reader not initialized".to_string()))
            }
        }
        else
        {
            Err(Error::new(io::ErrorKind::InvalidData, "Could not lock mutex".to_string()))
        }
    }
}