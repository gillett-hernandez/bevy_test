use std::time::Duration;

use bevy::prelude::*;

use crate::{
    ai::AI, bullet::Bullet, enemy::Enemy, events::BulletFired, misc::Lifetime, physics::Physics,
    player::Player, GameState,
};

#[derive(Component)]
pub struct GunData {
    pub timer: Timer,
    pub gun_type: GunType,
    sprite_handle: Handle<Image>,
    automatic: bool,
    bullet_velocity: Vec3,
    bullet_gravity: Vec3,
    bullet_friction: f32,
    bullet_duration: Duration,
}

impl GunData {
    pub fn new(
        handle: Handle<Image>,
        gun_type: GunType,
        shoot_cooldown: Duration,
        automatic: bool,
        bullet_velocity: Vec3,
        bullet_gravity: Vec3,
        bullet_friction: f32,
        bullet_duration: Duration,
    ) -> Self {
        GunData {
            timer: Timer::new(shoot_cooldown, false),
            gun_type,
            sprite_handle: handle,
            automatic,
            bullet_velocity,
            bullet_gravity,
            bullet_friction,
            bullet_duration,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GunType {
    SlugGun,
    MachineGun,
    Laser,
}

impl GunType {
    pub fn data_from_type(self, handle: Handle<Image>) -> GunData {
        match self {
            GunType::SlugGun => GunData::new(
                handle,
                self,
                Duration::from_millis(500),
                true,
                Vec3::new(0.0, 600.0, 0.0),
                Vec3::new(0.0, -4.0, 0.0),
                0.99,
                Duration::from_millis(2000),
            ),
            GunType::MachineGun => GunData::new(
                handle,
                self,
                Duration::from_millis(100),
                true,
                Vec3::new(0.0, 2500.0, 0.0),
                Vec3::new(0.0, -3.0, 0.0),
                0.95,
                Duration::from_millis(600),
            ),
            GunType::Laser => {
                unimplemented!()
            }
        }
    }
}

// #[derive(Component)]
// pub struct SlugGun {
//     timer: Timer,
//     sprite_handle: Handle<Image>,
//     bullet_velocity: Vec3,
//     bullet_gravity: Vec3,
//     bullet_friction: f32,
//     bullet_duration: Duration,
// }

// impl SlugGun {
//     pub fn new(handle: Handle<Image>) -> Self {
//         SlugGun {
//             timer: Timer::new(Duration::from_millis(500), true),
//             sprite_handle: handle,
//             bullet_velocity: Vec3::new(0.0, 600.0, 0.0),
//             bullet_gravity: Vec3::new(0.0, -4.0, 0.0),
//             bullet_duration: Duration::from_millis(2000),
//             bullet_friction: 0.99,
//         }
//     }
// }

// #[derive(Component)]
// pub struct MachineGun {
//     timer: Timer,
//     sprite_handle: Handle<Image>,
//     bullet_velocity: Vec3,
//     bullet_gravity: Vec3,
//     bullet_friction: f32,
//     bullet_duration: Duration,
// }

// impl MachineGun {
//     pub fn new(handle: Handle<Image>) -> Self {
//         // high speed but high friction bullets.
//         MachineGun {
//             timer: Timer::new(Duration::from_millis(100), true),
//             sprite_handle: handle,
//             bullet_velocity: Vec3::new(0.0, 2500.0, 0.0),
//             bullet_gravity: Vec3::new(0.0, -3.0, 0.0),
//             bullet_duration: Duration::from_millis(600),
//             bullet_friction: 0.95,
//         }
//     }
// }

fn gun_fire_system(
    mut commands: Commands,
    mut event_reader: EventReader<BulletFired>,
    query: Query<(Entity, &Physics, &Transform, &GunData)>,
    // asset_server: Res<AssetServer>,
) {
    if query.is_empty() {
        // QUESTION: should this actually panic instead?
        // if there's events present but there's no entity that can respond to those events.
        // or in more concrete terms,
        return;
    }
    // [x] fixed: event reader is not scoped or filtered to only handle this gun type
    // fixed by qualifying the event type with a templated custom event type.
    for event in event_reader.iter() {
        let (_e, physics, transform, gun) = query.get(event.entity).unwrap();
        commands
            .spawn_bundle((
                GlobalTransform::identity(),
                transform
                    .clone()
                    .with_translation(transform.translation - Vec3::Z), // change Z for sprite so that this draws above the background
                Bullet::<true> {
                    damage: 50.0,
                    piercing: true,
                },
                Lifetime::new(gun.bullet_duration),
                Physics {
                    velocity: physics.velocity + transform.rotation * gun.bullet_velocity,
                    gravity: gun.bullet_gravity,
                    friction: gun.bullet_friction,
                },
            ))
            .with_children(|child_builder| {
                // scale down bullet. this is because many bullets of different sizes will share the same sprite.
                child_builder.spawn_bundle(SpriteBundle {
                    texture: gun.sprite_handle.clone(),
                    // transform: Transform {
                    //     scale: Vec3::splat(0.2),
                    //     ..Default::default()
                    // },
                    ..Default::default()
                });
            });
    }
}
fn gun_input_system(
    // mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    // game: ResMut<Game>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Physics, &mut Transform, &mut GunData), With<Player>>,
    mut event_writer: EventWriter<BulletFired>,
    // config: Res<Assets<Config>>,
) {
    if query.is_empty() {
        return;
    }
    let (entity, _physics, _transform, mut gun) = query.single_mut();
    if ((gun.automatic && keyboard_input.pressed(KeyCode::Space))
        || (!gun.automatic && keyboard_input.just_pressed(KeyCode::Space)))
        && gun.timer.tick(time.delta()).finished()
    {
        // fire bullet
        event_writer.send(BulletFired::new(entity, false, gun.gun_type));
        gun.timer.reset();
    }
}

fn enemy_gun_system(
    time: Res<Time>,
    mut query: Query<(Entity, &mut GunData, &AI), With<Enemy>>,
    mut event_writer: EventWriter<BulletFired>,
) {
    for (entity, mut gun, ai) in query.iter_mut() {
        if ai.should_fire_bullet() && gun.timer.tick(time.delta()).finished() {
            event_writer.send(BulletFired::new(entity, false, gun.gun_type));
            gun.timer.reset();
        }
    }
}

// fn machine_gun_fire_system(
//     mut commands: Commands,
//     mut event_reader: EventReader<BulletFired<MachineGun>>,
//     query: Query<(Entity, &Physics, &Transform, &MachineGun)>,
//     // asset_server: Res<AssetServer>,
// ) {
//     if query.is_empty() {
//         // QUESTION: should this actually panic instead? see above QUESTION
//         return;
//     }

//     for event in event_reader.iter() {
//         let (_e, physics, transform, gun) = query.get(event.entity).unwrap();
//         commands
//             .spawn_bundle((
//                 GlobalTransform::identity(),
//                 transform
//                     .clone()
//                     .with_translation(transform.translation - Vec3::Z), // change Z for sprite so that this draws above the background
//                 Bullet::<true> {
//                     damage: 5.0,
//                     piercing: false,
//                 },
//                 Lifetime::new(gun.bullet_duration),
//                 Physics {
//                     velocity: physics.velocity + transform.rotation * gun.bullet_velocity,
//                     gravity: gun.bullet_gravity,
//                     friction: gun.bullet_friction,
//                 },
//             ))
//             .with_children(|child_builder| {
//                 child_builder.spawn_bundle(SpriteBundle {
//                     texture: gun.sprite_handle.clone(),
//                     // transform: Transform {
//                     //     scale: Vec3::splat(0.1),
//                     //     ..Default::default()
//                     // },
//                     ..Default::default()
//                 });
//             });
//     }
// }
// fn machine_gun_input_system(
//     // mut commands: Commands,
//     keyboard_input: Res<Input<KeyCode>>,
//     // game: ResMut<Game>,
//     time: ResMut<Time>,
//     mut query: Query<(Entity, &mut Physics, &mut Transform, &mut MachineGun), With<Player>>,
//     mut event_writer: EventWriter<BulletFired<MachineGun>>,
//     // config: Res<Assets<Config>>,
// ) {
//     if query.is_empty() {
//         return;
//     }
//     let (entity, _physics, _transform, mut gun) = query.single_mut();
//     if keyboard_input.pressed(KeyCode::Space) && gun.timer.tick(time.delta()).finished() {
//         // fire bullet
//         event_writer.send(BulletFired::new(entity, false));
//         gun.timer.reset();
//     }
// }

pub struct GunCollectionPlugin {}

impl Plugin for GunCollectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(gun_fire_system)
                .with_system(gun_input_system)
                .with_system(enemy_gun_system)
                // .with_system(slug_gun_fire_system)
                // .with_system(slug_gun_input_system),
        );
    }
}
