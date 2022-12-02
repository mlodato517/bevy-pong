use bevy::prelude::*;

#[derive(Component)]
struct Paddle(usize);

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct CeilingAndFloor;

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct Velocity(Vec3);

const BALL_SPEED: f32 = 50.0;
const BALL_SIZE: f32 = 20.0;
const WALL_THICKNESS: f32 = 10.0;
const PADDLE_SPEED: f32 = 200.0;
const PADDLE_THICKNESS: f32 = 20.0;
const PADDLE_HEIGHT: f32 = 80.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(setup)
        .add_system(move_ball)
        .add_system(move_players)
        .add_system(
            check_ball_against_ceiling_and_floor
                .after(move_ball)
                .after(move_players),
        )
        .add_system(
            check_ball_against_paddles
                .after(move_ball)
                .after(move_players),
        )
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Walls
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                rect: None,
                ..default()
            },
            transform: Transform::from_xyz(-300.0, 0.0, 0.0).with_scale(Vec3::new(
                WALL_THICKNESS,
                610.0,
                0.0,
            )),
            ..default()
        })
        .insert(Wall);
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                rect: None,
                ..default()
            },
            transform: Transform::from_xyz(300.0, 0.0, 0.0).with_scale(Vec3::new(
                WALL_THICKNESS,
                610.0,
                0.0,
            )),
            ..default()
        })
        .insert(Wall);
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                rect: None,
                ..default()
            },
            transform: Transform::from_xyz(0.0, -300.0, 0.0)
                .with_scale(Vec3::new(WALL_THICKNESS, 600.0, 0.0))
                .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
            ..default()
        })
        .insert(CeilingAndFloor);
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                rect: None,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 300.0, 0.0)
                .with_scale(Vec3::new(WALL_THICKNESS, 600.0, 0.0))
                .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
            ..default()
        })
        .insert(CeilingAndFloor);

    // Player 1
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                rect: None,
                ..default()
            },
            transform: Transform::from_xyz(-250.0, 0.0, 0.0).with_scale(Vec3::new(
                PADDLE_THICKNESS,
                PADDLE_HEIGHT,
                0.0,
            )),
            ..default()
        })
        .insert(Paddle(0));

    // Player 2
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                rect: None,
                ..default()
            },
            transform: Transform::from_xyz(250.0, 0.0, 0.0).with_scale(Vec3::new(
                PADDLE_THICKNESS,
                PADDLE_HEIGHT,
                0.0,
            )),
            ..default()
        })
        .insert(Paddle(1));

    // Ball
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                rect: None,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::ONE * BALL_SIZE),
            ..default()
        })
        .insert(Velocity(Vec3::new(-1.0, -2.0, 0.0)))
        .insert(Ball);
}

fn move_ball(mut ball_query: Query<(&mut Transform, &Velocity), With<Ball>>, time: Res<Time>) {
    let delta = time.delta_seconds();
    let (mut ball_transform, ball_velocity) = ball_query.single_mut();

    ball_transform.translation += ball_velocity.0 * delta * BALL_SPEED;
}

fn check_ball_against_ceiling_and_floor(
    mut ball_query: Query<(&Transform, &mut Velocity), With<Ball>>,
    ceilings_and_floors_query: Query<&Transform, With<CeilingAndFloor>>,
) {
    const THRESHOLD: f32 = BALL_SIZE / 2.0 + WALL_THICKNESS / 2.0;

    let (ball_transform, mut ball_velocity) = ball_query.single_mut();

    for boundary in ceilings_and_floors_query.iter() {
        let vert_distance = (ball_transform.translation.y - boundary.translation.y).abs();
        if vert_distance <= THRESHOLD {
            ball_velocity.0.y *= -1.0;
        }
    }
}

fn check_ball_against_paddles(
    mut ball_query: Query<(&Transform, &mut Velocity), With<Ball>>,
    paddles: Query<&Transform, With<Paddle>>,
) {
    const HORIZONTAL_THRESHOLD: f32 = BALL_SIZE / 2.0 + PADDLE_THICKNESS / 2.0;
    const VERTICAL_THRESHOLD: f32 = BALL_SIZE / 2.0 + PADDLE_HEIGHT / 2.0;

    let (ball_transform, mut ball_velocity) = ball_query.single_mut();

    for paddle in paddles.iter() {
        let horizontal_distance = (ball_transform.translation.x - paddle.translation.x).abs();
        if horizontal_distance <= HORIZONTAL_THRESHOLD {
            let vertical_distance = (ball_transform.translation.y - paddle.translation.y).abs();
            if vertical_distance <= VERTICAL_THRESHOLD {
                ball_velocity.0.x *= -1.0;
            }
        }
    }
}

struct Keys {
    up: KeyCode,
    down: KeyCode,
}
const PLAYER_KEYS: [Keys; 2] = [
    Keys {
        up: KeyCode::W,
        down: KeyCode::S,
    },
    Keys {
        up: KeyCode::Up,
        down: KeyCode::Down,
    },
];
fn move_players(
    mut paddles: Query<(&mut Transform, &Paddle)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();
    for (mut transform, player) in paddles.iter_mut() {
        let up_key = PLAYER_KEYS[player.0].up;
        if input.pressed(up_key) {
            transform.translation.y += delta * PADDLE_SPEED;
        }
        let down_key = PLAYER_KEYS[player.0].down;
        if input.pressed(down_key) {
            transform.translation.y -= delta * PADDLE_SPEED;
        }
    }
}
