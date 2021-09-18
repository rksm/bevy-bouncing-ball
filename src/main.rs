use bevy::{input::system::exit_on_esc_system, prelude::*};

pub fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            width: 400.0,
            height: 300.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::ALICE_BLUE))
        .add_plugins(DefaultPlugins)
        .add_startup_system_to_stage(StartupStage::PreStartup, setup_assets.system())
        .add_startup_system(setup.system())
        .add_system(step.system())
        .add_system(exit_on_esc_system.system())
        .run();
}

struct Floor;

struct Ball;

struct Velocity {
    value: f32,
}

struct Materials {
    ball_material: Handle<ColorMaterial>,
    floor_material: Handle<ColorMaterial>,
}

fn setup_assets(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(Materials {
        ball_material: materials.add(asset_server.load("sprites/soccer-ball-small.png").into()),
        floor_material: materials.add(Color::rgba(0.3, 1.0, 0.5, 1.0).into()),
    });
}

fn setup(mut commands: Commands, materials: Res<Materials>) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform.translation.y = 100.0;
    commands.spawn_bundle(camera);

    commands.spawn().insert(Floor).insert_bundle(SpriteBundle {
        sprite: Sprite::new((420.0, 100.0).into()),
        material: materials.floor_material.clone(),
        ..Default::default()
    });

    commands
        .spawn()
        .insert(Ball)
        .insert(Velocity { value: 0.0 })
        .insert_bundle(SpriteBundle {
            sprite: Sprite::new((70.0, 70.0).into()),
            material: materials.ball_material.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 200.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        });
}

fn step(time: Res<Time>, mut ball_query: Query<(&mut Transform, &mut Velocity), With<Ball>>) {
    if let Ok((mut tfm, mut v)) = ball_query.single_mut() {
        static GRAVITY: f32 = 9.81;
        let dt = time.delta_seconds() * 6.0;
        let y = tfm.translation.y;
        let y_new = y + v.value * dt - 0.5 * GRAVITY * dt.powi(2);
        let v_new = v.value - GRAVITY * dt;
        tfm.translation.y = y_new;
        v.value = if y_new < 0.0 { -v_new } else { v_new };
        println!("{} {} {}", v.value, y_new, dt);
    }
}
