use bevy::prelude::*;
use leafwing_input_manager::{prelude::{InputMap, ActionState, InputManagerPlugin}, Actionlike, InputManagerBundle};

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum PlayerAction {
    RightTrigger,
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    
    app.add_plugins(InputManagerPlugin::<PlayerAction>::default());
    app.add_systems(Startup, (setup,));
    app.add_systems(Update, (log_trigger_levels,));

    app.run()
}

fn setup(mut cmds: Commands) {
    let input_map =
        InputMap::default().insert(GamepadButtonType::RightTrigger2, PlayerAction::RightTrigger).build();

    cmds.spawn(InputManagerBundle::<PlayerAction>{
        input_map,
        ..default()
    });
}

fn log_trigger_levels(inputs: Query<&ActionState<PlayerAction>>) {
    info!("Trigger Level: {}", inputs.single().action_data(PlayerAction::RightTrigger).value);
}
