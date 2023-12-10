use bevy::prelude::*;

use crate::{movement::velocity_moves_transforms, swatter::swatter_follows_mouse};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        /*
        app
        .add_systems(
            Update,
            visualize_colliders
                .after(swatter_follows_mouse)
                .after(velocity_moves_transforms),
        )
        */
    }
}

/// Collider for collision detection (sorry, only circles because game jam)
#[derive(Component, Default)]
pub struct Collider {
    pub radius: f32,
}

impl Collider {
    // Check if this collider collides with another
    pub fn collides_with(
        &self,
        self_transform: &Transform,
        other: &Collider,
        other_transform: &Transform,
    ) -> bool {
        let distance = self_transform
            .translation
            .xy()
            .distance(other_transform.translation.xy());
        let radii = self.radius + other.radius;

        distance <= radii
    }
}

pub fn visualize_colliders(q: Query<(&Collider, &Transform)>, mut gizmos: Gizmos) {
    for (c, t) in q.iter() {
        gizmos
            .circle_2d(t.translation.xy(), c.radius, Color::ORANGE)
            .segments(64);
    }
}
