use crate::obstacle::*;
use crate::Sprite;
use crate::SpriteID;
use crate::{Rect, Rectf};
use std::collections::HashMap;

pub struct Wall {
    rect: Rect,
}

#[derive(Clone)]
pub struct Mobile {
    pub rect: Rect,
    pub sprite_id: SpriteID,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum ColliderID {
    Static(usize),
    Dynamic(usize),
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Side {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Contact {
    a: ColliderID,
    b: ColliderID,
    mtv: (i32, i32),
    side_a: Side,
}

pub struct Collision {}

impl Collision {
    fn rect_displacement(r1: Rectf, r2: Rect) -> Option<(f32, f32, Side)> {
        // Draw this out on paper to double check, but these quantities
        // will both be positive exactly when the conditions in rect_touching are true.
        let x_overlap = (r1.x + r1.w as f32).min(r2.x as f32 + r2.w as f32) - r1.x.max(r2.x as f32);
        let y_overlap = (r1.y + r1.h as f32).min(r2.y as f32 + r2.h as f32) - r1.y.max(r2.y as f32);
        if x_overlap >= 0.0 && y_overlap >= 0.0 {
            // This will return the magnitude of overlap in each axis.
            if x_overlap < y_overlap {
                if r1.x < r2.x as f32 {
                    Some((x_overlap, y_overlap, Side::Right))
                } else {
                    Some((x_overlap, y_overlap, Side::Left))
                }
            } else if r1.y < r2.y as f32 {
                Some((x_overlap, y_overlap, Side::Bottom))
            } else {
                Some((x_overlap, y_overlap, Side::Top))
            }
        } else {
            None
        }
    }

    // check mobiles against mobiles
    fn rectf_displacement(r1: Rectf, r2: Rectf) -> Option<(f32, f32, Side)> {
        // Draw this out on paper to double check, but these quantities
        // will both be positive exactly when the conditions in rect_touching are true.
        let x_overlap = (r1.x + r1.w as f32).min(r2.x + r2.w as f32) - r1.x.max(r2.x);
        let y_overlap = (r1.y + r1.h as f32).min(r2.y + r2.h as f32) - r1.y.max(r2.y);
        if x_overlap >= 0.0 && y_overlap >= 0.0 {
            // This will return the magnitude of overlap in each axis.
            if x_overlap < y_overlap {
                if r1.x < r2.x {
                    Some((x_overlap, y_overlap, Side::Right))
                } else {
                    Some((x_overlap, y_overlap, Side::Left))
                }
            } else if r1.y < r2.y {
                Some((x_overlap, y_overlap, Side::Bottom))
            } else {
                Some((x_overlap, y_overlap, Side::Top))
            }
        } else {
            None
        }
    }

    // Here we will be using push() on into, so it can't be a slice
    pub fn gather_contacts(
        statics: &Vec<Obstacle>,
        dynamics: &Vec<Sprite>,
        into: &mut Vec<Contact>,
    ) {
        // collide mobiles against mobiles
        for (ai, a) in dynamics.iter().enumerate() {
            for (bi, b) in dynamics.iter().enumerate().skip(ai + 1) {
                let displacement = Collision::rectf_displacement(a.rect, b.rect);
                if let Some(disp) = displacement {
                    into.push(Contact {
                        a: ColliderID::Dynamic(ai),
                        b: ColliderID::Dynamic(bi),
                        mtv: (disp.0 as i32, disp.1 as i32),
                        side_a: disp.2,
                    })
                }
            }
        }
        // collide mobiles against walls
        for (ai, a) in dynamics.iter().enumerate() {
            for (bi, b) in statics.iter().enumerate() {
                if let Some(rect) = b.rect {
                    let displacement = Collision::rect_displacement(a.rect, rect);
                    if let Some(disp) = displacement {
                        into.push(Contact {
                            a: ColliderID::Dynamic(ai),
                            b: ColliderID::Static(bi),
                            mtv: (disp.0 as i32, disp.1 as i32),
                            side_a: disp.2,
                        })
                    }
                }
            }
        }
    }

    fn restitute_dd(ai: usize, bi: usize, dynamics: &mut Vec<Sprite>, contact: &mut Contact) {
        match contact.side_a {
            Side::Top => {
                dynamics[ai].rect.y += (contact.mtv.1 / 2) as f32;
                dynamics[bi].rect.y -= (contact.mtv.1 / 2) as f32;
                // if displacing opposite to velocity, set y velocity to 0
                if dynamics[ai].vy < 0.0 {
                    dynamics[ai].vy = 0.0;
                }
                if dynamics[bi].vy > 0.0 {
                    dynamics[bi].vy = 0.0;
                }
            }
            Side::Bottom => {
                dynamics[ai].rect.y -= (contact.mtv.1 / 2) as f32;
                dynamics[bi].rect.y += (contact.mtv.1 / 2) as f32;
                if dynamics[ai].vy > 0.0 {
                    dynamics[ai].vy = 0.0;
                }
                if dynamics[bi].vy < 0.0 {
                    dynamics[bi].vy = 0.0;
                }
            }
            Side::Left => {
                dynamics[ai].rect.x += (contact.mtv.1 / 2) as f32;
                dynamics[bi].rect.x -= (contact.mtv.1 / 2) as f32;
                if dynamics[ai].vx < 0.0 {
                    dynamics[ai].vx = 0.0;
                }
                if dynamics[bi].vx > 0.0 {
                    dynamics[bi].vx = 0.0;
                }
            }
            Side::Right => {
                dynamics[ai].rect.x -= (contact.mtv.1 / 2) as f32;
                dynamics[bi].rect.x += (contact.mtv.1 / 2) as f32;
                if dynamics[ai].vx < 0.0 {
                    dynamics[ai].vx = 0.0;
                }
                if dynamics[bi].vx > 0.0 {
                    dynamics[bi].vx = 0.0;
                }
            }
        }
    }

    fn restitute_ds(ai: usize, dynamics: &mut Vec<Sprite>, contact: &mut Contact) {
        match contact.side_a {
            Side::Top => {
                dynamics[ai].rect.y += contact.mtv.1 as f32;
                // if displacing opposite to velocity, set y velocity to 0
                if dynamics[ai].vy < 0.0 {
                    dynamics[ai].vy = 0.0;
                }
            }
            Side::Bottom => {
                dynamics[ai].rect.y -= contact.mtv.1 as f32;
                if dynamics[ai].vy > 0.0 {
                    dynamics[ai].vy = 0.0;
                }
            }
            Side::Left => {
                dynamics[ai].rect.x += contact.mtv.1 as f32;
                if dynamics[ai].vx < 0.0 {
                    dynamics[ai].vx = 0.0;
                }
            }
            Side::Right => {
                dynamics[ai].rect.x -= contact.mtv.1 as f32;
                if dynamics[ai].vx < 0.0 {
                    dynamics[ai].vx = 0.0;
                }
            }
        }
    }

    pub fn restitute(
        statics: &Vec<Obstacle>,
        dynamics: &mut Vec<Sprite>,
        contacts: &mut Vec<Contact>,
    ) {
        // handle restitution of dynamics against dynamics and dynamics against statics wrt contacts.
        // You could instead make contacts `Vec<Contact>` if you think you might remove contacts.
        // You could also add an additional parameter, a slice or vec representing how far we've displaced each dynamic, to avoid allocations if you track a vec of how far things have been moved.
        // You might also want to pass in another &mut Vec<Contact> to be filled in with "real" touches that actually happened.
        contacts.sort_unstable_by_key(|c| -(c.mtv.0 * c.mtv.0 + c.mtv.1 * c.mtv.1));
        // Keep going!  Note that you can assume every contact has a dynamic object in .a.
        // You might decide to tweak the interface of this function to separately take dynamic-static and dynamic-dynamic contacts, to avoid a branch inside of the response calculation.
        // Or, you might decide to calculate signed mtvs taking direction into account instead of the unsigned displacements from rect_displacement up above.  Or calculate one MTV per involved entity, then apply displacements to both objects during restitution (sorting by the max or the sum of their magnitudes)
        // let mut restituted: HashMap<ColliderID, Mobile> = HashMap::new();
        let mut restituted: HashMap<ColliderID, ColliderID> = HashMap::new();
        for contact in contacts {
            if let ColliderID::Dynamic(ai) = contact.a {
                match contact.b {
                    ColliderID::Dynamic(bi) => {
                        if !restituted.contains_key(&contact.a) {
                            Collision::restitute_dd(ai, bi, dynamics, contact);
                            restituted.insert(contact.a, ColliderID::Dynamic(ai));
                            restituted.insert(contact.b, ColliderID::Dynamic(bi));
                        } else if let Some(disp) =
                            Collision::rectf_displacement(dynamics[ai].rect, dynamics[bi].rect)
                        {
                            contact.mtv = (disp.0 as i32, disp.1 as i32);
                            contact.side_a = disp.2;
                            Collision::restitute_dd(ai, bi, dynamics, contact);
                        }
                    }
                    ColliderID::Static(bi) => {
                        if !restituted.contains_key(&contact.a) {
                            Collision::restitute_ds(ai, dynamics, contact);
                            restituted.insert(contact.a, ColliderID::Dynamic(ai));
                        } else if let Some(rect) = statics[bi].rect {
                            if let Some(disp) =
                                Collision::rect_displacement(dynamics[ai].rect, rect)
                            {
                                contact.mtv = (disp.0 as i32, disp.1 as i32);
                                contact.side_a = disp.2;
                                Collision::restitute_ds(ai, dynamics, contact);
                            }
                        }
                    }
                }
            }
        }
    }
}
