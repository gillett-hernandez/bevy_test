use std::time::Duration;

use bevy::prelude::*;

pub mod bullet;
pub mod laser;

use crate::{
    enemy::Enemy,
    events::WeaponFired,
    input::Intent,
    misc::{CollisionRadius, Lifetime},
    physics::Physics,
    player::Player,
};

pub use bullet::{enemy_bullet_collision_system, player_bullet_collision_system, Bullet};
pub use laser::{enemy_laser_collision_system, Laser};

use crate::gamestate::GameState;

pub struct WeaponSubsystemPlugin;

impl Plugin for WeaponSubsystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(player_bullet_collision_system)
                .with_system(enemy_bullet_collision_system)
                .with_system(enemy_laser_collision_system),
        );
    }
}

pub enum WeaponSubtype {
    BulletBased {
        velocity: Vec3,
        gravity: Vec3,
        bullet_mass: f32,
        friction: f32,
        bullet_scale: f32,
    },
    Laser {
        width: f32,
        max_dist: f32,
    },
}

#[derive(Component)]
pub struct WeaponData {
    pub timer: Timer,
    pub weapon_type: WeaponType,
    pub sprite_handle: Handle<Image>,
    pub damage: f32,
    pub automatic: bool,
    // angle spread
    pub spread: f32,
    pub piercing: u32, // QUESTION: maybe change this f32 to represent some chance to pierce? i.e. 50% chance to pierce for each target hit.
    // note, we're tracking player hostility on the bullets, not on the gun. enemy-spawned bullets are hostile to the player, player-spawned bullets are not.
    pub lifetime: Duration,
    // TODO: think about whether this game will ever have 2 player vs or co-op.
    // if there's VS, then player hostility would need to be reworked to just reference the original entity and make sure collisions are ignored when they involve the bullet hitting the original entity.
    pub subtype: WeaponSubtype,
}

impl WeaponData {
    pub fn new_bullet_subtype(
        handle: Handle<Image>,
        weapon_type: WeaponType,
        shoot_cooldown: Duration,
        automatic: bool,
        bullet_damage: f32,
        bullet_spread: f32,
        bullet_velocity: Vec3,
        bullet_gravity: Vec3,
        bullet_friction: f32,
        bullet_lifetime: Duration,
        bullet_scale: f32,
        mass: f32,
        piercing: u32,
    ) -> Self {
        WeaponData {
            timer: Timer::new(shoot_cooldown, TimerMode::Repeating),
            weapon_type,
            sprite_handle: handle,
            automatic,
            damage: bullet_damage,
            spread: bullet_spread,
            lifetime: bullet_lifetime,
            piercing,
            subtype: WeaponSubtype::BulletBased {
                bullet_scale,
                velocity: bullet_velocity,
                gravity: bullet_gravity,
                friction: bullet_friction,
                bullet_mass: mass,
            },
        }
    }
    pub fn new_laser_subtype(
        handle: Handle<Image>,
        weapon_type: WeaponType,
        shoot_cooldown: Duration,
        automatic: bool,
        damage: f32,
        spread: f32,
        lifetime: Duration,
        width: f32,
        max_dist: f32,
        piercing: u32,
    ) -> Self {
        WeaponData {
            sprite_handle: handle,
            timer: Timer::new(shoot_cooldown, TimerMode::Repeating),
            piercing,
            weapon_type,
            automatic,
            damage,
            spread,
            lifetime,
            subtype: WeaponSubtype::Laser { width, max_dist },
        }
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub enum WeaponType {
    #[default]
    MachineGun,
    // SpreadGun,
    // Shotgun,
    // PulseGun,
    SlugGun,
    Laser,
    Gungine, // do not show
}

impl WeaponType {
    pub fn data_from_type_and_handle(self, handle: Handle<Image>) -> WeaponData {
        match self {
            WeaponType::SlugGun => WeaponData::new_bullet_subtype(
                handle,
                self,
                Duration::from_millis(500),
                true,
                20.0,
                0.1,
                Vec3::new(0.0, 800.0, 0.0),
                Vec3::new(0.0, -4.0, 0.0),
                0.9995,
                Duration::from_millis(2000),
                1.0,
                0.00005,
                10,
            ),
            WeaponType::Gungine => WeaponData::new_bullet_subtype(
                handle,
                self,
                Duration::from_millis(250),
                true,
                20.0,
                0.1,
                Vec3::new(0.0, -800.0, 0.0),
                Vec3::new(0.0, -4.0, 0.0),
                0.9995,
                Duration::from_millis(2000),
                1.0,
                0.00005,
                2,
            ),
            WeaponType::MachineGun => WeaponData::new_bullet_subtype(
                handle,
                self,
                Duration::from_millis(100),
                true,
                50.0,
                0.3,
                Vec3::new(0.0, 1000.0, 0.0),
                Vec3::new(0.0, -3.0, 0.0),
                0.995,
                Duration::from_millis(600),
                0.6,
                0.000005, // very low mass
                0,
            ),
            WeaponType::Laser => WeaponData::new_laser_subtype(
                handle,
                self,
                Duration::from_millis(10),
                true,
                5.0,
                0.0,
                Duration::from_millis(20),
                15.0,
                f32::INFINITY,
                5,
            ),
        }
    }
}

fn gun_fire_system(
    mut commands: Commands,
    mut event_reader: EventReader<WeaponFired>,
    query: Query<(Entity, &Transform, &WeaponData)>,
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
        let (_e /*, physics */, transform, weapon) = query.get(event.entity).unwrap();

        assert!(event.weapon_type == weapon.weapon_type);
        // for example a triplicate gun would fire groups of 3 bullets with spread, and a shotgun would fire a spread of bullets randomly.

        let mut bundle = SpatialBundle::default();
        bundle.transform.translation = transform.translation;

        match weapon.subtype {
            WeaponSubtype::BulletBased {
                velocity,
                gravity,
                bullet_mass,
                friction,
                bullet_scale,
            } => {
                // single fire per event
                let angle = weapon.spread * (rand::random::<f32>() - 0.5);
                commands
                    .spawn(bundle)
                    .insert((
                        Bullet {
                            damage: weapon.damage,
                            piercing: weapon.piercing,
                            hostile_to_player: event.hostile,
                        },
                        CollisionRadius(bullet_scale * 10.0),
                        Lifetime::new(weapon.lifetime),
                        Physics {
                            mass: bullet_mass,
                            velocity: event.entity_velocity
                                + transform.rotation.mul_quat(Quat::from_rotation_z(angle))
                                    * velocity,
                            gravity,
                            friction,
                        },
                    ))
                    .with_children(|child_builder| {
                        // scale down bullet. this is because many bullets of different sizes will share the same sprite.
                        child_builder.spawn(SpriteBundle {
                            texture: weapon.sprite_handle.clone(),
                            transform: Transform {
                                scale: Vec3::splat(bullet_scale),
                                translation: Vec3::new(0.0, 0.0, 1.0), // change Z for sprite so that this draws above the background
                                ..Default::default()
                            },
                            ..Default::default()
                        });
                    });
            }
            WeaponSubtype::Laser { width, max_dist } => {
                commands
                    .spawn(bundle)
                    .insert((
                        Laser::new(weapon.damage, event.hostile, width, max_dist),
                        Lifetime::new(weapon.lifetime),
                        Transform {
                            // overwrite transform
                            scale: Vec3::new(1.0, 20.0, 1.0),
                            translation: transform.translation
                                + transform.rotation * Vec3::new(0.0, 200.0, 1.0), // change Z for sprite so that this draws above the background
                            rotation: transform.rotation,
                        },
                    ))
                    .with_children(|child_builder| {
                        // scale down bullet. this is because many bullets of different sizes will share the same sprite.
                        child_builder.spawn(SpriteBundle {
                            texture: weapon.sprite_handle.clone(),
                            ..Default::default()
                        });
                    });
            }
        }
    }
}

fn player_gun_system(
    time: Res<Time>,
    mut query: Query<
        (
            Entity,
            &mut Physics,
            &mut Transform,
            &mut WeaponData,
            &Intent,
        ),
        With<Player>,
    >,
    mut event_writer: EventWriter<WeaponFired>,
) {
    if query.is_empty() {
        return;
    }
    let (entity, physics, _transform, mut weapon, intent) = query.single_mut();
    if weapon.timer.tick(time.delta()).finished()
        && ((weapon.automatic && intent.fire) || (!weapon.automatic && intent.just_fired))
    {
        // fire bullet
        event_writer.send(WeaponFired::new(
            entity,
            physics.velocity,
            false,
            weapon.weapon_type,
        ));
        weapon.timer.reset();
    }
}

fn enemy_gun_system(
    time: Res<Time>,
    mut query: Query<(Entity, &mut WeaponData, &Physics, &Intent), With<Enemy>>,
    mut event_writer: EventWriter<WeaponFired>,
) {
    for (entity, mut weapon, physics, intent) in query.iter_mut() {
        if intent.fire && weapon.timer.tick(time.delta()).finished() {
            event_writer.send(WeaponFired::new(
                entity,
                physics.velocity,
                true,
                weapon.weapon_type,
            ));
            weapon.timer.reset();
        }
    }
}

pub struct GunCollectionPlugin;

impl Plugin for GunCollectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(gun_fire_system)
                .with_system(player_gun_system)
                .with_system(enemy_gun_system), // .with_system(slug_gun_fire_system)
                                                // .with_system(slug_gun_input_system),
        );
    }
}
