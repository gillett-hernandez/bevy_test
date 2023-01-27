
use bevy::prelude::*;
#[derive(Component)]
pub struct TakesContactDamage {}

#[derive(Component)]
pub struct DealsContactDamage {}

pub fn contact_damage_system<Takers: Component, Dealers: Component>(
    mut query_damage_takers: Query<(Entity, &mut Takers, &TakesContactDamage)>,
    query_damage_dealers: Query<(Entity, &Dealers, &DealsContactDamage)>,
) {
    // check collisions between takers and dealers.
    // ideally we would have an acceleration structure here to make collision checks faster
}
