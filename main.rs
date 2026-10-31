use std::thread::{self, sleep};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::key_macros::MacroChannels;
use crate::{journal::{GameState, JournalReader}};
use winit::window::Window;
use winit::{application::ApplicationHandler, event::{self, DeviceEvent, RawKeyEvent}, event_loop::{self, ActiveEventLoop, EventLoop} };
use winit::event::{ElementState, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};
use std::sync::mpsc;

mod journal;
mod key_macros;

#[derive(Default)]
struct App
{
    game_state_mutex: Arc<Mutex<GameState>>,
    macro_channels: MacroChannels
}

impl App
{
    fn new() -> App
    {
        App {
            game_state_mutex: Arc::new(Mutex::new(GameState::new())),
            macro_channels: MacroChannels::default(),
        }
    }

    fn init(&mut self)
    {
        // splash screen
        println!("--------------------------------------------");
        println!("FOXTROT AUTOPILOT v0.1.3");
        println!("by 74oshua");
        println!("");
        println!("Press F3 to toggle Fleet Carrier Autopilot");
        println!("Ensure the correct route has been imported into SrvSurvey");
        println!("Ensure you're in a ship with cargo space if you will need to load tritium");
        println!("Note: You may have leftover tritium in your inventory when the route is complete");
        println!("--------------------------------------------");

        // spawn thread to continuously update journal
        let game_state = Arc::clone(&self.game_state_mutex);
        thread::spawn(move || {
            let game_state = &game_state;
            if let Ok(mut journal_reader) = JournalReader::new()
            {
                let mut current = false;
                loop
                {
                    if let Err(_) = journal_reader.update_journal(&game_state)
                    {
                        print!("Error reading journal");
                    }
                    if !current
                    {
                        if let Ok(gs) = game_state.lock()
                        {
                            if gs.current
                            {
                                current = true;
                            }
                        }
                    }
                    else
                    {
                        sleep(Duration::from_millis(10));
                    }
                }
            }
        });

        // wait for journal to be current
        loop
        {
            if let Ok(game_state) = self.game_state_mutex.lock()
            {
                if game_state.current
                {
                    break;
                }
            }
            sleep(Duration::from_millis(100));
        }
        println!("Journal up to date");
    }
}

impl ApplicationHandler for App
{
    fn resumed(&mut self, _event_loop: &ActiveEventLoop)
    {
        println!("Program initializing...");
        self.init();
    }
    
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
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
        _event_loop: &ActiveEventLoop,
        _device_id: event::DeviceId,
        event: event::DeviceEvent,
    )
    {
        match event
        {
            // if F2 pressed
            // toggle trade macro
            DeviceEvent::Key (
                RawKeyEvent { physical_key: PhysicalKey::Code(KeyCode::F3), state: ElementState::Released }
             ) =>
             {
                let mut run_macro = true;
                // if already running, cancel
                if let Some(ref mut trade_macro_cancel) = self.macro_channels.fc_route_cancel
                {
                    if trade_macro_cancel.send(true).is_ok()
                    {
                        run_macro = false;
                    }
                    self.macro_channels.fc_route_cancel = None;
                    println!("Macro cancelled");
                }
                // otherwise, run
                if run_macro
                {
                    let game_state = Arc::clone(&self.game_state_mutex);
                    let (tx, rx) = mpsc::channel();
                    self.macro_channels.fc_route_cancel = Some(tx);
                    thread::spawn(
                        move || {
                            println!("Running FC macro...");
                            key_macros::fc_auto_route(&game_state, rx);
                            println!("FC macro ended");
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
}