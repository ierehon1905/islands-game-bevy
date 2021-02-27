use bevy::{
    math::{vec2, vec3, Vec3Swizzles},
    prelude::*,
};
use rand::{prelude::SliceRandom, Rng};

use crate::{house::House, island::Island, Materials};

const PERSON_SPEED: f32 = 10.;

pub struct WanderTimer(Timer);
pub struct TargetPosition(Option<Vec2>);
pub struct Person {
    name: String,
}

const AVAILABLE_PERSON_NAMES: [&str; 7] = [
    "Leon", "Alina", "Elena", "Eduard", "Alexey", "Michael", "Vasya",
];

pub fn colonize_homes(
    commands: &mut Commands,
    materials: Res<Materials>,
    query: Query<Entity, With<House>>,
) {
    for house in query.iter() {
        println!("Spawning person");
        let person_e = commands
            .spawn(SpriteBundle {
                material: materials.skin.clone(),
                sprite: Sprite::new(Vec2::new(5.0, 10.0)),
                // transform: Transform::from_translation(vec3(x, 0., 0.)),
                ..Default::default()
            })
            .with(Person {
                name: AVAILABLE_PERSON_NAMES
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .to_string(),
            })
            .with(TargetPosition(None))
            .current_entity()
            .unwrap();

        commands.push_children(house, &[person_e]);
    }
}

pub fn move_people(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut TargetPosition, &Person), With<Person>>,
) {
    // For every person
    let delta = time.delta_seconds() * PERSON_SPEED;
    for (mut trans, mut optional_target, person) in query.iter_mut() {
        // println!("Moving pal named: {:?}", person.1.name.clone());
        // If there is a place they want to go
        if let Some(target) = optional_target.0 {
            // MAKE THE NORMALIZED DIR VEC
            let dir = vec3(
                target.x - trans.translation.x,
                target.y - trans.translation.y,
                0.,
            );
            if dir.length_squared() < 4. {
                optional_target.0 = None;
                println!("{} has reached destination", person.name);
                continue;
            }
            let dir = delta * dir.normalize();

            trans.translation += dir;
        }
    }
}

pub fn make_people_wander(
    time: Res<Time>,
    mut timer: ResMut<WanderTimer>,
    mut query: Query<(&mut TargetPosition, &Person), With<Person>>,
) {
    // update our timer with the time elapsed since the last update
    // if the timer hasn't finished yet, we return
    if !timer.0.tick(time.delta_seconds()).just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();

    for mut person in query.iter_mut() {
        let x: f32 = rng.gen_range(-100.0..=100.0);
        let y: f32 = rng.gen_range(-100.0..=100.0);
        // rng.gen_range(0..1);
        // println!("{} wants to go to {};{}!", person.1.name, x, y);
        *person.0 = TargetPosition(Some(vec2(x, y)))
    }
}

pub struct PeoplePlugin;
impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut AppBuilder) {
        // the reason we call from_seconds with the true flag is to make the timer repeat itself
        app.add_resource(WanderTimer(Timer::from_seconds(2.0, true)))
            // .add_startup_system(add_people.system())
            .add_system(make_people_wander.system())
            .add_system(move_people.system());
    }
}
