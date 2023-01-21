use std::time::Duration;

use bevy::prelude::*;

pub mod bullet;
pub mod laser;

use crate::{
    ai::AI, enemy::Enemy, events::GunFired, misc::Lifetime, physics::Physics, player::Player,
};

pub use bullet::{
    enemy_bullet_collision_system, player_bullet_collision_system, Bullet, BulletCollisionPlugin,
};
pub use laser::{Laser, LaserCollisionPlugin};

use crate::gamestate::GameState;

pub struct WeaponSubsystemPlugin;

impl Plugin for WeaponSubsystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(player_bullet_collision_system)
                .with_system(enemy_bullet_collision_system),
        );
    }
}

#[derive(Component)]
pub struct GunData {
    pub timer: Timer,
    pub gun_type: GunType,
    sprite_handle: Handle<Image>,
    automatic: bool,
    velocity: Vec3,
    gravity: Vec3,
    friction: f32,
    lifetime: Duration,
    scale: f32,
    bullet_mass: f32,
    piercing: u32, // QUESTION: maybe change this f32 to represent some chance to pierce? i.e. 50% chance to pierce for each target hit.
                   // note, we're tracking player hostility on the bullets, not on the gun. enemy-spawned bullets are hostile to the player, player-spawned bullets are not.
                   // TODO: think about whether this game will ever have 2 player vs or co-op.
                   // if there's VS, then player hostility would need to be reworked to just reference the original entity and make sure collisions are ignored when they involve the bullet hitting the original entity.
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
        bullet_lifetime: Duration,
        bullet_scale: f32,
        mass: f32,
        piercing: u32,
    ) -> Self {
        GunData {
            timer: Timer::new(shoot_cooldown, TimerMode::Repeating),
            gun_type,
            sprite_handle: handle,
            automatic,
            velocity: bullet_velocity,
            gravity: bullet_gravity,
            friction: bullet_friction,
            lifetime: bullet_lifetime,
            scale: bullet_scale,
            bullet_mass: mass,
            piercing,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GunType {
    SlugGun,
    MachineGun,
    Gungine,
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
                Vec3::new(0.0, 800.0, 0.0),
                Vec3::new(0.0, -4.0, 0.0),
                0.9995,
                Duration::from_millis(2000),
                1.0,
                0.00005, // relatevely high mass
                10,
            ),
            GunType::Gungine => GunData::new(
                handle,
                self,
                Duration::from_millis(250),
                true,
                Vec3::new(0.0, -800.0, 0.0),
                Vec3::new(0.0, -4.0, 0.0),
                0.9995,
                Duration::from_millis(2000),
                1.0,
                0.00005, // relatevely high mass
                2,
            ),
            GunType::MachineGun => GunData::new(
                handle,
                self,
                Duration::from_millis(100),
                true,
                Vec3::new(0.0, 2000.0, 0.0),
                Vec3::new(0.0, -3.0, 0.0),
                0.995,
                Duration::from_millis(600),
                0.6,
                0.000005, // very low mass
                0,
            ),
            GunType::Laser => {
                unimplemented!()
            }
        }
    }
}

fn gun_fire_system(
    mut commands: Commands,
    mut event_reader: EventReader<GunFired>,
    query: Query<(Entity, &Physics, &Transform, &GunData)>,
    // asset_server: Res<AssetServer>,
) {
    if query.is_empty() {
        // QUESTION: should this actually panic instead?
        // if there's events present but there's no entity that can respond to those events.
        // or in more concrete terms,
        return;
    }

    for event in event_reader.iter() {
        // get entity properties for the owner of the gun that was fired
        let (_e, physics, transform, gun) = query.get(event.entity).unwrap();

        assert!(event.gun_type == gun.gun_type);
        // note: can do a match here based on gun type to conditionally spawn bullets in different ways based on the gun type.
        // for example a triplicate gun would fire groups of 3 bullets with spread, and a shotgun would fire a spread of bullets randomly.

        let mut bundle = SpatialBundle::default();
        bundle.transform.translation = transform.translation;

        match event.gun_type {
            GunType::SlugGun | GunType::MachineGun | GunType::Gungine => {
                commands
                    .spawn(bundle)
                    .insert((
                        Bullet {
                            mass: gun.bullet_mass,
                            piercing: gun.piercing,
                            hostile: event.hostile,
                        },
                        Lifetime::new(gun.lifetime),
                        Physics {
                            velocity: physics.velocity + transform.rotation * gun.velocity,
                            gravity: gun.gravity,
                            friction: gun.friction,
                        },
                    ))
                    .with_children(|child_builder| {
                        // scale down bullet. this is because many bullets of different sizes will share the same sprite.
                        child_builder.spawn(SpriteBundle {
                            texture: gun.sprite_handle.clone(),
                            transform: Transform {
                                scale: Vec3::splat(gun.scale),
                                translation: Vec3::new(0.0, 0.0, 1.0), // change Z for sprite so that this draws above the background
                                ..Default::default()
                            },
                            ..Default::default()
                        });
                    });
            }
            GunType::Laser => {
                commands
                    .spawn(bundle)
                    .insert((
                        Laser::new(event.hostile, 1.0),
                        Lifetime::new(gun.lifetime),
                        Physics {
                            velocity: physics.velocity + transform.rotation * gun.velocity,
                            gravity: gun.gravity,
                            friction: gun.friction,
                        },
                    ))
                    .with_children(|child_builder| {
                        // scale down bullet. this is because many bullets of different sizes will share the same sprite.
                        child_builder.spawn(SpriteBundle {
                            texture: gun.sprite_handle.clone(),
                            transform: Transform {
                                scale: Vec3::splat(gun.scale),
                                translation: Vec3::new(0.0, 0.0, 1.0), // change Z for sprite so that this draws above the background
                                ..Default::default()
                            },
                            ..Default::default()
                        });
                    });
            }
        }
    }
}

fn gun_input_system(
    // mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    // game: ResMut<Game>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Physics, &mut Transform, &mut GunData), With<Player>>,
    mut event_writer: EventWriter<GunFired>,
    // config: Res<Assets<Config>>,
) {
    if query.is_empty() {
        return;
    }
    let (entity, _physics, _transform, mut gun) = query.single_mut();
    if gun.timer.tick(time.delta()).finished()
        && ((gun.automatic && keyboard_input.pressed(KeyCode::Space))
            || (!gun.automatic && keyboard_input.just_pressed(KeyCode::Space)))
    {
        // fire bullet
        event_writer.send(GunFired::new(entity, false, gun.gun_type));
        gun.timer.reset();
    }
}

fn enemy_gun_system(
    time: Res<Time>,
    mut query: Query<(Entity, &mut GunData, &AI), With<Enemy>>,
    mut event_writer: EventWriter<GunFired>,
) {
    for (entity, mut gun, ai) in query.iter_mut() {
        if ai.should_fire_bullet() && gun.timer.tick(time.delta()).finished() {
            event_writer.send(GunFired::new(entity, true, gun.gun_type));
            gun.timer.reset();
        }
    }
}

pub struct GunCollectionPlugin;

impl Plugin for GunCollectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(gun_fire_system)
                .with_system(gun_input_system)
                .with_system(enemy_gun_system), // .with_system(slug_gun_fire_system)
                                                // .with_system(slug_gun_input_system),
        );
    }
}
