use bevy::prelude::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, visualize_colliders);
    }
}

/// Collider for collision detection (sorry, only circles because game jam)
#[derive(Component)]
pub struct Collider {
    pub radius: f32,
}


impl Collider {
    // Check if this collider collides with another
    pub fn collides_with(&self, self_transform: &Transform, other: &Self, other_transform: &Transform) -> bool {
        let dist_x = self_transform.translation.x - other_transform.translation.x;
        let dist_y = self_transform.translation.y - other_transform.translation.y;

        let distance: f32 = ((dist_x * dist_x) + (dist_y * dist_y)).sqrt();

        distance <= self.radius + other.radius
    }
}

pub fn visualize_colliders(q: Query<(&Collider, &Transform)>, mut gizmos: Gizmos) {
      for (c, t) in q.iter() {
        gizmos.circle_2d(t.translation.xy(), c.radius, Color::ORANGE);
    }
}