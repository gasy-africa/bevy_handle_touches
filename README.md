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

This example sets up a basic Bevy application and includes a system `handle_touches` that processes touch events. The `TouchEvent` struct contains information about the touch event, such as its phase (started, moved, or ended) and the position of the touch.

Make sure you have the `bevy` and `bevy_ios` crates in your dependencies section of `Cargo.toml`:

```toml
[dependencies]
bevy = "0.5"
bevy_ios = "0.5"
```

```
add state
```

To add state to the Bevy example for handling touch events, you can introduce a custom component to track the current state of the touch. Here's an updated version of the code that includes a TouchState component:

```rust
use bevy::prelude::*;
use bevy::app::Events;
use bevy::app::EventReader;

struct TouchState {
    active: bool,
    position: Option<Vec2>,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(handle_touches.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(SpriteBundle::default().insert(TouchState {
            active: false,
            position: None,
        }));
}

fn handle_touches(
    mut commands: Commands,
    mut state: Local<TouchState>,
    mut events: EventReader<TouchEvent>,
    mut query: Query<(&mut Transform, &mut TouchState)>,
) {
    for event in events.iter() {
        if let Some(touch) = event.touches.get(0) {
            match event.phase {
                TouchPhase::Started => {
                    state.active = true;
                    state.position = Some(touch.position);
                }
                TouchPhase::Moved => {
                    state.position = Some(touch.position);
                }
                TouchPhase::Ended => {
                    state.active = false;
                    state.position = None;
                }
                _ => {}
            }
        }
    }

    for (mut transform, mut touch_state) in query.iter_mut() {
        if state.active {
            if let Some(position) = state.position {
                // Update the sprite position or perform other actions based on touch state
                transform.translation = Vec3::new(position.x, position.y, 0.0);
            }
        }
        touch_state.active = state.active;
        touch_state.position = state.position;
    }
}
```

In this updated code, we introduce a `TouchState` component that includes an `active` field to track whether a touch is currently active and a position field to store the touch `position`. We spawn a sprite entity with the TouchState component to visualize its state.

The `handle_touches` system updates the `TouchState` based on touch events and applies the corresponding transformations to the sprite entity. The touch state is then propagated to the `TouchState` component of the sprite entity.

Feel free to modify the code further based on your specific requirements and game logic.
