use bevy::{
    ecs::Stage, input::mouse::MouseButtonInput, math::vec3, prelude::*, render::camera::Camera,
};
use bevy_prototype_lyon::prelude::*;

mod island;
use island::*;

mod house;
use house::*;
mod person;
use person::{colonize_homes, move_people, PeoplePlugin};
use resource::ResourcesPlugin;

mod resource;

const CAMERA_SPEED: f32 = 10.;

pub struct Materials {
    pub soil_material: Handle<ColorMaterial>,
    pub grass_material: Handle<ColorMaterial>,
    pub house_material: Handle<ColorMaterial>,
    pub transparent: Handle<ColorMaterial>,
    pub skin: Handle<ColorMaterial>,
}
#[derive(Debug, Default)]
pub struct Selected {
    pub items: Vec<Entity>,
}

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn(Camera2dBundle::default())
        .insert_resource(Materials {
            soil_material: materials.add(Color::RED.into()),
            grass_material: materials.add(Color::GREEN.into()),
            house_material: materials.add(Color::BLUE.into()),
            skin: materials.add(Color::PINK.into()),
            transparent: materials.add(Color::rgba_linear(0., 0., 0., 0.5).into()),
        });
}

fn cam_move(keys: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Camera>>) {
    for mut cam in query.iter_mut() {
        // Keyboard input
        if keys.pressed(KeyCode::A) {
            cam.translation.x -= CAMERA_SPEED;
        } else if keys.pressed(KeyCode::D) {
            cam.translation.x += CAMERA_SPEED;
        }
        if keys.pressed(KeyCode::W) {
            cam.translation.y += CAMERA_SPEED;
        } else if keys.pressed(KeyCode::S) {
            cam.translation.y -= CAMERA_SPEED;
        }
    }
}

fn my_cursor_system(
    commands: &mut Commands,
    // events to get cursor position
    ev_cursor: Res<Events<CursorMoved>>,
    mut evr_cursor: Local<EventReader<CursorMoved>>,
    ev_mousebtn: Res<Events<MouseButtonInput>>,
    mut evr_mousebtn: Local<EventReader<MouseButtonInput>>,
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    mut q_camera: QuerySet<(
        Query<&Transform, With<Camera>>,
        Query<(&mut Hovered, &GlobalTransform, &Parent)>,
    )>,
) {
    // Mouse buttons
    // for ev in evr_mousebtn.iter(&ev_mousebtn) {
    //     if ev.state.is_pressed() {
    //         eprintln!("Just pressed mouse button: {:?}", ev.button);
    //     } else {
    //         eprintln!("Just released mouse button: {:?}", ev.button);
    //     }
    // }

    // q_camera.q0_mut();
    // assuming there is exactly one main camera entity, so this is OK
    let camera_transform = q_camera.q0().iter().next().unwrap().compute_matrix();
    let islands = q_camera.q1_mut().iter_mut();
    if let Some(ev) = evr_cursor.latest(&ev_cursor) {
        let wnd = wnds.get(ev.id).unwrap();
        let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        let p = ev.position - size / 2.0;
        let pos_wld = camera_transform * p.extend(0.0).extend(1.0);
        let pos_wld = vec3(pos_wld.x, pos_wld.y, 0.);

        for (mut hovered, transform, p) in islands {
            let far = if (transform.translation.x - pos_wld.x).abs() > 100.
                || (transform.translation.y - pos_wld.y).abs() > 100.
            {
                true
            } else {
                false
            };

            // println!("dist {:?}", far);
            if !far {
                if let Some(eeee) = evr_mousebtn.latest(&ev_mousebtn) {
                    if !eeee.state.is_pressed() {
                        // println!("Clicked {:?}", eeee.button)
                    }
                }
                if !hovered.0 {
                    hovered.0 = true;
                }
            } else if hovered.0 {
                hovered.0 = false;
            }
        }
    }
}

fn draw_hovered_islands(
    commands: &mut Commands,
    materials: Res<Materials>,
    mut query: Query<(&Hovered, &mut Handle<ColorMaterial>), Changed<Hovered>>,
) {
    for (hovered, mut mat) in query.iter_mut() {
        // println!("Changed _______ {:?}, hovered: {:?}", hovered, mat);
        if hovered.0 {
            *mat = materials.soil_material.clone();
        } else {
            *mat = materials.transparent.clone();
        }
    }
}

enum MyStages {
    PreSetup,
    Islands,
    Homes,
    People,
}

// impl MyStages {
//     pub fn
// }

impl MyStages {
    pub fn to_str(self) -> &'static str {
        match self {
            MyStages::PreSetup => "pre_setup",
            MyStages::Islands => "islands",
            MyStages::Homes => "homes",
            MyStages::People => "people",
        }
    }
}

fn main() {
    App::build()
        .init_resource::<Selected>()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_stage(
            MyStages::PreSetup.to_str(),
            SystemStage::single(setup.system()),
        )
        // .add_startup_stage_after("pre_setup", "game_setup", SystemStage::serial())
        // .add_startup_system_to_stage("islands")
        .add_startup_stage_after(
            MyStages::PreSetup.to_str(),
            MyStages::Islands.to_str(),
            SystemStage::single(spawn_islands.system()),
        )
        .add_startup_stage_after(
            MyStages::Islands.to_str(),
            MyStages::Homes.to_str(),
            SystemStage::single(build_house.system()),
        )
        .add_startup_stage_after(
            MyStages::Homes.to_str(),
            MyStages::People.to_str(),
            SystemStage::single(colonize_homes.system()),
        )
        .add_system(cam_move.system())
        .add_system(my_cursor_system.system())
        .add_system(draw_hovered_islands.system())
        .add_plugin(PeoplePlugin)
        .add_plugin(ResourcesPlugin)
        .run();
}
