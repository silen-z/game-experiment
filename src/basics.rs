use bevy::prelude::*;
#[derive(Default)]
pub struct Velocity(pub Vec3);

impl From<Vec3> for Velocity {
    fn from(v: Vec3) -> Self {
        Self(v)
    }
}

pub struct Rotation(f32);

impl From<f32> for Rotation {
    fn from(v: f32) -> Self {
        Self(v)
    }
}
pub struct SpriteAnimation {
    timer: Timer,
    current: u32,
    frames: u32,
}

impl SpriteAnimation {
    pub fn new(millis: u32, frames: u32) -> Self {
        SpriteAnimation {
            timer: Timer::from_seconds(millis as f32 / 1000., true),
            current: 0,
            frames,
        }
    }

    fn next(&mut self) -> u32 {
        self.current = (self.current + 1) % self.frames;
        self.current
    }
}

pub struct Lifetime(Timer);

impl Lifetime {
    pub fn millis(millis: u32) -> Self {
        Self(Timer::from_seconds(millis as f32 / 1000., false))
    }

    pub fn seconds(seconds: u32) -> Self {
        Self(Timer::from_seconds(seconds as f32, false))
    }
}

pub struct MaximumDistanceFrom {
    pub anchor: Entity,
    pub distance: f32,
}

pub struct CameraFollow(pub Entity);

pub fn movement(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_seconds();
    }
}

pub fn continuous_rotation(time: Res<Time>, mut query: Query<(&Rotation, &mut Transform)>) {
    for (rotation, mut transform) in query.iter_mut() {
        let rotation = transform.rotation * Quat::from_rotation_z(rotation.0);
        transform.rotation = transform.rotation.slerp(rotation, time.delta_seconds());
    }
}

pub fn sprite_animation(
    mut query: Query<(&mut SpriteAnimation, &mut TextureAtlasSprite)>,
    time: Res<Time>,
) {
    for (mut anim, mut sprite) in query.iter_mut() {
        if anim.timer.tick(time.delta_seconds()).just_finished() {
            sprite.index = anim.next();
        }
    }
}

pub fn lifetime(
    commands: &mut Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Lifetime)>,
) {
    for (entity, mut lifetime) in query.iter_mut() {
        if lifetime.0.tick(time.delta_seconds()).finished() {
            commands.despawn(entity);
        }
    }
}

pub fn maximum_distance_from(
    cmd: &mut Commands,
    query: Query<(Entity, &Transform, &MaximumDistanceFrom)>,
    anchors: Query<&Transform>,
) {
    for (entity, transform, max_dist) in query.iter() {
        if let Ok(anchor_transform) = anchors.get(max_dist.anchor) {
            if transform.translation.distance(anchor_transform.translation) > max_dist.distance {
                cmd.despawn(entity);
            }
        } else {
            cmd.despawn(entity);
        }
    }
}

// pub fn camera_follow(
//     cmds: &mut Commands,
//     mut cameras: Query<(&CameraFollow, &mut Transform)>,
//     entities: Query<&Transform, Without<CameraFollow>>,
// ) {
//     for (camera_follow, mut camera) in cameras.iter_mut() {
//         if let Ok(followed) = entities.get(camera_follow.0) {
//             camera.translation.x = followed.translation.x;
//             camera.translation.y = followed.translation.y;
//         } else {
//             cmds.remove_one::<CameraFollow>(camera_follow.0);
//         }
//     }
// }

pub fn cleanup<C: Component>(cmd: &mut Commands, query: Query<Entity, With<C>>) {
    for entity in query.iter() {
        cmd.despawn(entity);
    }
}
