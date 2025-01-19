use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;
pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SIZE: f32 = 64.0;
pub const ENEMY_SPEED: f32 = 200.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_enemies)
        .add_systems(Update, player_movement)
        .add_systems(Update, confine_player_movement)
        .add_systems(Update, enemy_movement)
        .add_systems(Update, (update_enemy_direction, enemy_wall_collision).chain())
        .add_systems(Update, confine_enemy_movement)
        .add_systems(Update, enemy_hit_player)
        .add_event::<EnemyWallCollisionEvent>()
        .run();
}

#[derive(Component)]
struct Player {}

#[derive(Component)]
struct Enemy {
    pub direction: Vec2,
}

#[derive(Event, Default)]
struct EnemyWallCollisionEvent;

#[derive(Resource, Deref)]
struct EnemyWallCollisionSound(Handle<AudioSource>);

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
      Sprite::from_image(
            asset_server.load("sprites/ball_blue_large.png"),
        ),
        Player {},
    ));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(
      Camera2d
    );
}

fn spawn_enemies(
    mut commands: Commands,
    windows: Query<&Window>,
    asset_server: Res<AssetServer>,
) {
    let window = windows.single();

    let half_enemy_size = ENEMY_SIZE / 2.0;
    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * (window.width() - ENEMY_SIZE) - window.width() / 2.0 + half_enemy_size;
        let random_y = random::<f32>() * (window.height() - ENEMY_SIZE) - window.height() / 2.0 + half_enemy_size;

        commands.spawn((
            Sprite::from_image(
              asset_server.load("sprites/ball_red_large.png"),
            ),
            Transform::from_xyz(random_x, random_y, 0.0),
            Enemy {
              direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }

    let wall_collision_sound = asset_server.load("audio/pluck_001.ogg");
    commands.insert_resource(EnemyWallCollisionSound(wall_collision_sound));
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta().as_secs_f32();
    }
}

fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    windows: Query<&Window>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = windows.single();

        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = - window.width() / 2.0 + half_player_size;
        let x_max = window.width() / 2.0 - half_player_size;
        let y_min = - window.height() / 2.0 + half_player_size;
        let y_max = window.height() / 2.0 - half_player_size;

        let mut translation = player_transform.translation;


        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}


fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta().as_secs_f32();
    }
}

fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut collision_events: EventWriter<EnemyWallCollisionEvent>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = - window.width() / 2.0 + half_enemy_size;
    let x_max = window.width() / 2.0 - half_enemy_size;
    let y_min = - window.height() / 2.0 + half_enemy_size;
    let y_max = window.height() / 2.0 - half_enemy_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed = false;

        let translation = transform.translation;

        // Sometimes, enemys get stuck on an edge
        if translation.x > x_max {
            println!("edge! x_max: {} x {} direction {}", x_max, translation.x, enemy.direction.x);
        }
        if translation.y > y_max {
            println!("egdge! y_max: {} y {} direction {}", y_max, translation.y, enemy.direction.y);
        }

        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x = - enemy.direction.x;
            direction_changed = true;
        }

        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y = - enemy.direction.y;
            direction_changed = true;
        }

        if direction_changed {
            collision_events.send_default();
        }
    }
}

fn enemy_wall_collision(
    mut commands: Commands,
    mut collision_events: EventReader<EnemyWallCollisionEvent>,
    sound: Res<EnemyWallCollisionSound>,
) {
    if !collision_events.is_empty() {
        // This prevents events staying active on the next frame.
        collision_events.clear();
        commands.spawn((AudioPlayer(sound.clone()), PlaybackSettings::DESPAWN));
    }
}

fn confine_enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy), With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = - window.width() / 2.0 + half_enemy_size;
    let x_max = window.width() / 2.0 - half_enemy_size;
    let y_min = - window.height() / 2.0 + half_enemy_size;
    let y_max = window.height() / 2.0 - half_enemy_size;

    for (mut transform, enemy) in enemy_query.iter_mut() {
        let mut translation = transform.translation;

        if translation.x < x_min {
            translation.x = x_min + enemy.direction.x.abs();
        } else if translation.x > x_max {
            translation.x = x_max -  enemy.direction.x.abs();
        }
        if translation.y < y_min {
            translation.y = y_min + enemy.direction.y.abs();
        } else if translation.y > y_max {
            translation.y = y_max - enemy.direction.y.abs();
        }

        transform.translation = translation;
    }
}

fn enemy_hit_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            if distance < player_radius + enemy_radius {
                println!("Enemy hit player! Game Over!");
                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                AudioPlayer::<AudioSource>(sound_effect);
                commands.entity(player_entity).despawn();
            }
        }
    }
}
