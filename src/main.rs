use iyesengine::prelude::*;

fn main() {
    App::new()
        // includes bevy::DefaultPlugins and all the other stuff    
        .add_plugin(IyesEverything)
        // ... add more stuff
        .run();
}