use std::thread::{self, JoinHandle};
use std::sync::{Arc, Mutex};

use crate::key_macros::MacroChannels;
use crate::{journal::{GameState, JournalReader}};
use winit::{application::ApplicationHandler, event::{self, DeviceEvent, RawKeyEvent}, event_loop::{self, ActiveEventLoop, EventLoop}, keyboard::KeyCode::KeyC, window::{Window, WindowAttributes}};
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};
use std::sync::mpsc;

mod journal;
mod key_macros;

#[derive(Default)]
struct App
{
    // journal_reader: Option<JournalReader>,
    // game_state: GameState,
    // key_macros: KeyMacros,
    journal_reader: Option<JournalReader>,
    game_state_mutex: Arc<Mutex<GameState>>,
    macro_channels: MacroChannels
}

impl App
{
    fn new() -> App
    {
        App {
            // journal_reader: JournalReader::new().ok(), game_state: GameState::new(), key_macros: KeyMacros::new(),
            journal_reader: JournalReader::new().ok(), game_state_mutex: Arc::new(Mutex::new(GameState::new())),
            macro_channels: MacroChannels::default()
        }
    }

    fn init(&mut self)
    {
        // read journal to present
        if let Some(ref mut journal_reader) = self.journal_reader
        {
            // update game state according to journal until caught up
            while !journal_reader.is_current()
            {
                journal_reader.update_journal(Arc::clone(&self.game_state_mutex));
            }
            println!("Journal reader is up to date");
        }

        // spawn thread to continuously update journal
        let game_state = Arc::clone(&self.game_state_mutex);
        thread::spawn(move || {
            loop
            {
                if let Ok(mut journal_reader) = JournalReader::new()
                {
                    journal_reader.update_journal(Arc::clone(&game_state));
                }
            }
        });
    }
    
    fn update(&mut self)
    {
        // read journal to present
        if let Some(ref mut journal_reader) = self.journal_reader
        {
            loop
            {
                journal_reader.update_journal(Arc::clone(&self.game_state_mutex));
            }
        }
    }
}

impl ApplicationHandler for App
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop)
    {
        println!("Program initializing...");
        self.init();
    }
    
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: event::WindowEvent,
    )
    {
        match event
        {
            WindowEvent::CloseRequested =>
            {
                event_loop.exit();
            },
            _ => ()
        }
    }

    fn device_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        device_id: event::DeviceId,
        event: event::DeviceEvent,
    )
    {
        match event
        {
            // if F2 pressed
            // toggle trade macro
            DeviceEvent::Key (
                RawKeyEvent { physical_key: PhysicalKey::Code(KeyCode::F2), state: ElementState::Released }
             ) =>
             {
                println!("F2 pressed");
                // if already running, cancel
                if let Some(ref mut trade_macro_cancel) = self.macro_channels.fc_route_cancel
                {
                    let _ = trade_macro_cancel.send(true);
                    self.macro_channels.fc_route_cancel = None;
                    println!("Trade macro cancelled");
                }
                // otherwise, run
                else
                {
                    let game_state = Arc::clone(&self.game_state_mutex);
                    let (tx, rx) = mpsc::channel();
                    self.macro_channels.fc_route_cancel = Some(tx);
                    thread::spawn(
                        move || {
                            println!("Running FC macro...");
                            key_macros::fc_auto_route(rx);
                            println!("Trade FC ended");
                        }
                    );
                }
             },
            _ => ()
        };
    }
}

fn main()
{
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new();
    event_loop.set_control_flow(event_loop::ControlFlow::Poll);
    event_loop.listen_device_events(event_loop::DeviceEvents::Always);
    event_loop.run_app(&mut app).unwrap();

    // // create journal reader
    // let mut journal_reader = JournalReader::new().unwrap();
    // let event_loop = EventLoop::new().unwrap();
    // event_loop.set_control_flow(event_loop::ControlFlow::Poll);
    // // init game state
    // let mut game_state = GameState::new();
    // loop
    // {
    //     // update game state according to journal
    //     journal_reader.update_journal(&mut game_state).unwrap();
    // }
}