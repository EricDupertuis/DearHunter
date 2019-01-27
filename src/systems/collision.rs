use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
};

use crate::components::{BoundingRect, Velocity, VelocityCmd};
use crate::states::{ARENA_HEIGHT, ARENA_WIDTH};
use crate::tree::Tree;


pub struct CollisionSystem;
impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, BoundingRect>,
        ReadStorage<'s, VelocityCmd>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Tree>,
    );

    fn run(&mut self, (mut velocities, rectangles, commands, transforms, trees): Self::SystemData) {
        // Map commands to velocities
        for (vel, cmd) in (&mut velocities, &commands).join() {
            vel.x = cmd.x;
            vel.y = cmd.y;
        }

        // Perform collision checks on arena boundaries
        for (brect, transform, vel) in (&rectangles, &transforms, &mut velocities).join() {
            let x = transform.translation().x;
            let y = transform.translation().y;

            let half_h = brect.height / 2.;
            let half_w = brect.width / 2.;

            if ((y + half_h) >= ARENA_HEIGHT && vel.y > 0.) || ((y - half_h) <= 0. && vel.y < 0.) {
                vel.y = 0.0;
            }

            if ((x + half_w) >= ARENA_WIDTH && vel.x > 0.) || ((x - half_w) <= 0. && vel.x < 0.) {
                vel.x = 0.0;
            }
        }

        // Perform collision checks with trees
        for (brect, transform, vel) in (&rectangles, &transforms, &mut velocities).join() {
            let p = Point {
                x: transform.translation().x,
                y: transform.translation().y,
            };

            for (_tree, tree_tf, tree_brect) in (&trees, &transforms, &rectangles).join() {
                let tree = Rect {
                    center: Point {
                        x: tree_tf.translation().x,
                        y: tree_tf.translation().y,
                    },
                    width: tree_brect.width,
                    height: tree_brect.height,
                };
                let thing = Rect {
                    center: Point {
                        x: transform.translation().x,
                        y: transform.translation().y,
                    },
                    width: brect.width,
                    height: brect.height,
                };

                if collides(&thing, &tree) {
                    let dx = tree.center.x - p.x;
                    let dy = tree.center.y - p.y;
                    if vel.x / dx > 0. {
                        vel.x *= -0.1;
                    }
                    if vel.y / dy > 0. {
                        vel.y *= -0.1;
                    }
                }
            }
        }
    }
}

pub struct Point {
    pub x: f32,
    pub y: f32,
}

struct Segment {
    pub min: f32,
    pub max: f32,
}
impl Segment {
    pub fn contains(&self, x: f32) -> bool {
        x >= self.min && x <= self.max
    }
}

pub struct Rect {
    pub center: Point,
    pub width: f32,
    pub height: f32,
}
impl Rect {
    pub fn contains(&self, p: &Point) -> bool {
        let collision_x = self.segment_x().contains(p.x);
        let collision_y = self.segment_y().contains(p.y);
        return collision_x && collision_y;
    }
    fn segment_x(&self) -> Segment {
        Segment {
            min: self.center.x - self.width / 2.,
            max: self.center.x + self.width / 2.,
        }
    }
    fn segment_y(&self) -> Segment {
        Segment {
            min: self.center.y - self.height / 2.,
            max: self.center.y + self.height / 2.,
        }
    }
    pub fn corners(&self) -> [Point; 4] {
        [
            Point {
                x: self.center.x - self.width / 2.,
                y: self.center.y - self.height / 2.,
            },
            Point {
                x: self.center.x + self.width / 2.,
                y: self.center.y - self.height / 2.,
            },
            Point {
                x: self.center.x + self.width / 2.,
                y: self.center.y + self.height / 2.,
            },
            Point {
                x: self.center.x - self.width / 2.,
                y: self.center.y + self.height / 2.,
            },
        ]
    }
}

pub fn collides(a: &Rect, b: &Rect) -> bool {
    for corner in a.corners().iter() {
        if b.contains(corner) {
            return true;
        }
    }
    false
}
