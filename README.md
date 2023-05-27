# bevy_handle_touches

```
cargo new bevy_handle_touches
```
>     Created binary (application) `bevy_handle_touches` package

```
cd bevy_handle_touches
```


> generate touch event in bevy for iOS

In the Bevy game engine, you can generate touch events for iOS using the `TouchEvent` struct and the input resource. Here's an example of how you can generate touch events in Bevy for iOS:

```rust
use bevy::prelude::*;
use bevy::app::Events;
use bevy::app::EventReader;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(handle_touches.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn handle_touches(
    mut events: EventReader<TouchEvent>,
    mut query: Query<&mut Transform>,
) {
    for event in events.iter() {
        if let Some(touch) = event.touches.get(0) {
            // Process the touch event here
            if event.phase == TouchPhase::Started {
                println!("Touch started at position: {:?}", touch.position);
            } else if event.phase == TouchPhase::Moved {
                println!("Touch moved to position: {:?}", touch.position);
            } else if event.phase == TouchPhase::Ended {
                println!("Touch ended at position: {:?}", touch.position);
            }
        }
    }
}
```
