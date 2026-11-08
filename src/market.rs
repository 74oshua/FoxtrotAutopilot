use std::sync::{Arc, Mutex};

use std::fs::File;
use std::io::{self, Error, Seek};
use serde::Deserialize;

use crate::GameState;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarketItem
{
    #[serde(rename(deserialize = "id"))]
    pub id: u32,
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "Name_Localised"))]
    pub name_local: String,
    #[serde(rename(deserialize = "Category"))]
    pub category: String,
    #[serde(rename(deserialize = "Category_Localised"))]
    pub category_local: String,
    #[serde(rename(deserialize = "Consumer"))]
    pub consumer: bool,
    #[serde(rename(deserialize = "Producer"))]
    pub producer: bool,
}

#[derive(Deserialize, Clone)]
pub struct Market
{
    #[serde(rename(deserialize = "StationName"))]
    pub station_name: String,
    #[serde(rename(deserialize = "StationType"))]
    pub station_type: String,
    #[serde(rename(deserialize = "Items"))]
    pub items: Vec<MarketItem>,
}

pub struct MarketReader
{
    market_iter: Option<io::BufReader<File>>
}


impl MarketReader
{
    pub fn new() -> io::Result<MarketReader>
    {
        // get home directory from $USERPROFILE
        if let Some(home_path) = std::env::home_dir()
        {
            // convert to string
            if let Some(home_str) = home_path.to_str()
            {
                // append journal directory to $USERPROFILE to get full journal path
                let mut market_dir: String = home_str.to_owned();
                market_dir.push_str("\\Saved Games\\Frontier Developments\\Elite Dangerous\\Market.json");

                // open newest journal file and return iterator
                println!("Found market file: {}", market_dir);
                let file = File::open(market_dir)?;
                Ok(MarketReader { market_iter: Some(io::BufReader::new(file)) })
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

    fn sort_market(game_state: &mut GameState)
    {
        if let Some(ref mut market) = game_state.market
        {
            // remove items not being sold
            market.items.retain(|item| item.consumer || item.producer);
            // sort in order items will appear in menu
            market.items.sort_unstable_by_key(|item| (item.category_local.clone(), item.name_local.clone()));
        }
    }

    pub fn update_market(&mut self, game_state_mut: &Arc<Mutex<GameState>>) -> Result<(), Error>
    {
        // lock game state
        if let Ok(mut game_state) = game_state_mut.lock()
        {
            // deserialize market
            if let Some(ref mut market_iter) = self.market_iter
            {
                // reset to beginning of file
                let _ = market_iter.seek(io::SeekFrom::Start(0));
                if let Ok(market) = serde_json::from_reader::<&mut io::BufReader<File>, Market>(market_iter)
                {
                    game_state.market = Some(market.clone());
                    MarketReader::sort_market(&mut game_state);
                }
            }
            Ok(())
        }
        else
        {
            Err(Error::new(io::ErrorKind::InvalidData, "Could not lock mutex".to_string()))
        }
    }
}