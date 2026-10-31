use std::sync::{Arc, Mutex};

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Error};
use glob::glob;
use serde::Deserialize;

// information about a fleet carrier
#[derive(Clone)]
pub enum CarrierType
{
    Unknown,
    FleetCarrier,
    SquadronCarrier
}

#[derive(Clone)]
pub struct Carrier
{
    pub carrier_type: CarrierType,
    pub name: String,
    pub callsign: String,
    pub star_system: String,
    pub fuel_level: u32
}

impl Default for Carrier
{
    fn default() -> Self
    {
        Carrier { carrier_type: CarrierType::Unknown, name: "".to_string(), callsign: "".to_string(), star_system: "".to_string(), fuel_level: 0 }
    }
}

// state of Elite Dangerous
#[derive(Clone)]
pub struct GameState
{
    pub fsd_target: String,
    pub music_track: String,
    pub carriers: HashMap<u32, Carrier>,
    pub last_event: String,
}

impl GameState
{
    pub fn new() -> GameState
    {
        GameState
        {
            fsd_target: "".to_string(),
            music_track: "".to_string(),
            carriers: HashMap::new(),
            last_event: "".to_string()
        }
    }
}

impl Default for GameState
{
    fn default() -> Self
    {
        GameState::new()
    }
}

// Elite Dangerous journal entry event struct
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct EDEvent
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
    fuel_level: Option<u32>
}

pub struct JournalReader
{
    journal_iter: Option<io::Lines<io::BufReader<File>>>,
    // whether or not the journal has caught up to the present
    current: bool,
}

impl Default for JournalReader
{
    fn default() -> Self
    {
        JournalReader { journal_iter: None, current: false }
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
                journal_dir.push_str("\\Saved Games\\Frontier Developments\\Elite Dangerous\\Journal.*.log");
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
                    let file = File::open(current_journal_name)?;
                    Ok(JournalReader { journal_iter: Some(io::BufReader::new(file).lines()), current: false })
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

    pub fn update_journal(&mut self, game_state_mut: Arc<Mutex<GameState>>) -> Result<(), Error>
    {
        if let Ok(mut game_state) = game_state_mut.lock()
        {
            // try to get the next line from the journal
            if let Some(ref mut iter) = self.journal_iter
            {
                if let Some(next_line) = iter.next()
                {
                    // testing: print it
                    // println!("{}", next_line?);

                    // get the line as an EDEvent
                    if let Ok(event) = serde_json::from_str::<EDEvent>(&next_line?[..])
                    {
                        // save last event
                        game_state.last_event = event.event.clone();

                        // test: print event
                        // println!("{}: {}", event.timestamp, event.event);
                        // set FSD target
                        if event.event == "FSDTarget"
                        {
                            if let Some(fsd_target) = event.name
                            {
                                game_state.fsd_target = fsd_target;
                            }
                            else
                            {
                                println!("error getting FSD target");
                            }
                        }
                        else if event.event == "Music"
                        {
                            if let Some(music_track) = event.music_track
                            {
                                game_state.music_track = music_track;
                            }
                            else
                            {
                                println!("error getting music track");
                            }
                        }
                        else if event.event == "CarrierStats" || event.event == "CarrierLocation"
                        {
                            if let Some(carrier_id) = event.carrier_id
                            {
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
                            }
                        }
                    }
                }
                else
                {
                    // if self.current == false
                    // {
                    //     for carrier in game_state.carriers.values()
                    //     {
                    //         println!("Carrier Name: {}, Callsign: {}, Location: {}, Fuel Level: {}/1000", carrier.name, carrier.callsign, carrier.star_system, carrier.fuel_level);
                    //     }
                    // }
                    self.current = true;
                }
                Ok(())
            }
            else
            {
                Err(Error::new(io::ErrorKind::InvalidData, "journal reader not initialized".to_string()))
            }
        }
        else
        {
            Err(Error::new(io::ErrorKind::InvalidData, "could not lock mutex".to_string()))
        }
    }

    pub fn is_current(&self) -> bool
    {
        self.current
    }
}