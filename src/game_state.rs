use std::collections::HashMap;
use crate::market::Market;

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
    pub target_system: Option<String>,
    pub fuel_level: u32
}

impl Default for Carrier
{
    fn default() -> Self
    {
        Carrier { carrier_type: CarrierType::Unknown, name: "".to_string(), callsign: "".to_string(), star_system: "".to_string(), target_system: None, fuel_level: 0 }
    }
}

// state of Elite Dangerous
#[derive(Clone)]
pub struct GameState
{
    pub current: bool,
    pub market: Option<Market>,
    pub fsd_target: Option<String>,
    pub music_track: String,
    pub docked: bool,
    pub docked_station_type: Option<String>,
    pub station_name: Option<String>,
    pub docked_carrier_id: Option<u32>,
    pub carriers: HashMap<u32, Carrier>,
    pub last_event: String,
}

impl GameState
{
    pub fn new() -> GameState
    {
        GameState
        {
            current: false,
            market: None,
            fsd_target: None,
            music_track: "".to_string(),
            docked: false,
            docked_station_type: None,
            station_name: None,
            docked_carrier_id: None,
            carriers: HashMap::new(),
            last_event: "".to_string(),
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