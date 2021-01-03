use bevy::prelude::*;
use bevy::render::pass::ClearColor;

// Constants

const WINDOW_WIDTH: f32 = 1500.0;
const WINDOW_HEIGHT: f32 = 800.0;

const PADDLE_WIDTH: f32 = 30.0;
const PADDLE_HEIGHT: f32 = 120.0;
const PADDLE_SPEED: f32 = 1000.0;
const PADDLE_INIT_X: f32 = 650.0;

// Structs

struct PlayerPaddle;
struct PaddleVars {
  paddle_max_y: f32,
  paddle_min_y: f32,
  paddle_color: Handle<ColorMaterial>,
}

// Main

fn main() {
  App::build()
    .add_resource(WindowDescriptor {
      title: "Pong".to_string(),
      width: WINDOW_WIDTH,
      height: WINDOW_HEIGHT,
      ..Default::default()
    })
    .add_startup_system(setup.system())
    .add_startup_stage("setup", SystemStage::single(spawn_player_paddle.system()))
    .add_system(player_paddle_movement.system())
    .add_plugins(DefaultPlugins)
    .add_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
    .run();
}

// Systems

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
  commands.spawn(Camera2dBundle::default());

  let paddle_half = PADDLE_HEIGHT / 2.0;
  let range = (WINDOW_HEIGHT / 2.0) - paddle_half;

  commands.insert_resource(PaddleVars {
    paddle_color: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
    paddle_max_y: -range,
    paddle_min_y: range,
  });
}

fn spawn_player_paddle(commands: &mut Commands, materials: Res<PaddleVars>) {
  commands
    .spawn(SpriteBundle {
      material: materials.paddle_color.clone(),
      transform: Transform::from_translation(Vec3::new(-PADDLE_INIT_X, 0.0, 0.0)),
      sprite: Sprite::new(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
      ..Default::default()
    })
    .with(PlayerPaddle);
}

fn player_paddle_movement(
  time: Res<Time>,
  keyboard_input: Res<Input<KeyCode>>,
  mut query: Query<&mut Transform, With<PlayerPaddle>>,
  paddle_vars: Res<PaddleVars>,
) {
  for mut transform in query.iter_mut() {
    let mut direction = 0.0;
    if keyboard_input.pressed(KeyCode::Up) {
      direction += 1.0;
    } else if keyboard_input.pressed(KeyCode::Down) {
      direction -= 1.0;
    }

    let translation = &mut transform.translation;
    translation.y += time.delta_seconds() * direction * PADDLE_SPEED;
    translation.y = translation
      .y
      .min(paddle_vars.paddle_min_y)
      .max(paddle_vars.paddle_max_y);
  }
}
