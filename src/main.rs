use bevy::prelude::*;
use bevy::input::touch::*;

struct TouchEvent {
    ev: TouchInput,
}

fn main() {
        // create a "constructor" closure, which can initialize
    // our data and move it into a closure that bevy can run as a system
    let mut config = TouchEvent {
            ev: TouchInput { 
                    phase: TouchPhase::Started
                , position: Vec2 { x: 0.0, y: 0.0 }
                , force: None, id: 0 
            }
        };

    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(setup)
        .add_system(move |events: EventReader<TouchInput>| {
            // call our function from inside the closure
            handle_touches(events, &mut config );
        })
        .run();
}

fn setup(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());
}

fn handle_touches(
    mut events: EventReader<TouchInput>,
    state: &mut TouchEvent,
) {
    for ev in events.iter() {
        // in real apps you probably want to store and track touch ids somewhere
        match ev.phase {
            TouchPhase::Started => {
                println!("Touch {} started at: {:?}", ev.id, ev.position);
                state.ev = *ev;
            }
            TouchPhase::Moved => {
                println!("Touch {} moved to: {:?}", ev.id, ev.position);
            }
            TouchPhase::Ended => {
                println!("Touch {} ended at: {:?}", ev.id, ev.position);
                println!("State {} ended at: {:?}", state.ev.id, state.ev.position);
                state.ev = TouchInput { 
                    phase: TouchPhase::Started
                    , position: Vec2 { x: 0.0, y: 0.0 }
                    , force: None, id: 0 
                }
            }
            TouchPhase::Cancelled => {
                println!("Touch {} cancelled at: {:?}", ev.id, ev.position);
            }
        }    
    }
}