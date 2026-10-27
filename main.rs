use crate::{journal::{GameState, JournalReader}};
use winit::{application::ApplicationHandler, event::{self, DeviceEvent, RawKeyEvent}, event_loop::{self, ActiveEventLoop, EventLoop}, keyboard::KeyCode::KeyC, window::{Window, WindowAttributes}};
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};
use key_macros::KeyMacros;

mod journal;
mod key_macros;

#[derive(Default)]
struct App
{
    journal_reader: Option<JournalReader>,
    game_state: GameState,
    key_macros: KeyMacros
}

impl App
{
    fn new() -> App
    {
        App { journal_reader: JournalReader::new().ok(), game_state: GameState::new(), key_macros: KeyMacros::new() }
    }

    fn init(&mut self)
    {
        // init game state
        let mut game_state = GameState::new();
        if let Some(ref mut journal_reader) = self.journal_reader
        {
            // update game state according to journal until caught up
            while !journal_reader.is_current()
            {
                journal_reader.update_journal(&mut game_state).unwrap();
            }
            println!("Journal reader is up to date");
        }
    }
    
    fn update(&mut self)
    {
        println!("Updating");
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
            DeviceEvent::Key (
                RawKeyEvent { physical_key: PhysicalKey::Code(KeyCode::F2), state: ElementState::Released }
             ) =>
             {
                println!("Running trade macro...");
                if let Some(ref mut journal_reader) = self.journal_reader
                {
                    self.key_macros.auto_trade(&mut self.game_state, journal_reader, 2, "Ngurii".to_string(), "Hyades Sector FL-O b6-2".to_string());
                }
                else
                {
                    println!("journal error");
                }
                println!("Trade macro done");
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