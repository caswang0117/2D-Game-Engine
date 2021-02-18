use num::signum;
use pixels::{Pixels, SurfaceTexture};
use std::collections::HashMap;
use std::time::Instant;
use winit::dpi::PhysicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
​
// seconds per frame
const DT: f64 = 1.0 / 60.0;
​
const DEPTH: usize = 4;
const WIDTH: usize = 320;
const HEIGHT: usize = 240;
const PITCH: usize = WIDTH * DEPTH;
​
// We'll make our Color type an RGBA8888 pixel.
type Color = [u8; DEPTH];
​
const CLEAR_COL: Color = [32, 32, 64, 255];
const WALL_COL: Color = [200, 200, 200, 255];
const PLAYER_COL: Color = [255, 128, 128, 255];
​
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Rect {
    x: i32,
    y: i32,
    w: u16,
    h: u16,
}
​
struct Wall {
    rect: Rect,
}
​
#[derive(Clone, Copy)]
struct Mobile {
    rect: Rect,
    vx: f32,
    vy: f32,
}
​
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum ColliderID {
    Static(usize),
    Dynamic(usize),
}
​
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Side {
    Left,
    Right,
    Top,
    Bottom,
}
​
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
struct Contact {
    a: ColliderID,
    b: ColliderID,
    mtv: (i32, i32),
    side_a: Side,
}
​
// pixels gives us an rgba8888 framebuffer
fn clear(fb: &mut [u8], c: Color) {
    // Four bytes per pixel; chunks_exact_mut gives an iterator over 4-element slices.
    // So this way we can use copy_from_slice to copy our color slice into px very quickly.
    for px in fb.chunks_exact_mut(4) {
        px.copy_from_slice(&c);
    }
}
​
#[allow(dead_code)]
fn rect(fb: &mut [u8], r: Rect, c: Color) {
    assert!(r.x < WIDTH as i32);
    assert!(r.y < HEIGHT as i32);
    // NOTE, very fragile! will break for out of bounds rects!  See next week for the fix.
    let x1 = (r.x + r.w as i32).min(WIDTH as i32) as usize;
    let y1 = (r.y + r.h as i32).min(HEIGHT as i32) as usize;
    for row in fb[(r.y as usize * PITCH)..(y1 * PITCH)].chunks_exact_mut(PITCH) {
        for p in row[(r.x as usize * DEPTH)..(x1 * DEPTH)].chunks_exact_mut(DEPTH) {
            p.copy_from_slice(&c);
        }
    }
}
​
fn rect_displacement(r1: Rect, r2: Rect) -> Option<(i32, i32, Side)> {
    // Draw this out on paper to double check, but these quantities
    // will both be positive exactly when the conditions in rect_touching are true.
    let x_overlap = (r1.x + r1.w as i32).min(r2.x + r2.w as i32) - r1.x.max(r2.x);
    let y_overlap = (r1.y + r1.h as i32).min(r2.y + r2.h as i32) - r1.y.max(r2.y);
    if x_overlap >= 0 && y_overlap >= 0 {
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
​
// Here we will be using push() on into, so it can't be a slice
fn gather_contacts(statics: &[Wall], dynamics: &[Mobile], into: &mut Vec<Contact>) {
    // collide mobiles against mobiles
    for (ai, a) in dynamics.iter().enumerate() {
        for (bi, b) in dynamics.iter().enumerate().skip(ai + 1) {
            let displacement = rect_displacement(a.rect, b.rect);
            if let Some(disp) = displacement {
                into.push(Contact {
                    a: ColliderID::Dynamic(ai),
                    b: ColliderID::Dynamic(bi),
                    mtv: (disp.0, disp.1),
                    side_a: disp.2,
                })
            }
        }
    }
    // collide mobiles against walls
    for (ai, a) in dynamics.iter().enumerate() {
        for (bi, b) in statics.iter().enumerate() {
            let displacement = rect_displacement(a.rect, b.rect);
            if let Some(disp) = displacement {
                into.push(Contact {
                    a: ColliderID::Dynamic(ai),
                    b: ColliderID::Static(bi),
                    mtv: (disp.0, disp.1),
                    side_a: disp.2,
                })
            }
        }
    }
}
​
fn restitute_dd(ai: usize, bi: usize, dynamics: &mut [Mobile], contact: &mut Contact) {
    match contact.side_a {
        Side::Top => {
            dynamics[ai].rect.y += contact.mtv.1 / 2;
            dynamics[bi].rect.y -= contact.mtv.1 / 2;
            // if displacing opposite to velocity, set y velocity to 0
            if dynamics[ai].vy < 0.0 {
                dynamics[ai].vy = 0.0;
            }
            if dynamics[bi].vy > 0.0 {
                dynamics[bi].vy = 0.0;
            }
        }
        Side::Bottom => {
            dynamics[ai].rect.y -= contact.mtv.1 / 2;
            dynamics[bi].rect.y += contact.mtv.1 / 2;
            if dynamics[ai].vy > 0.0 {
                dynamics[ai].vy = 0.0;
            }
            if dynamics[bi].vy < 0.0 {
                dynamics[bi].vy = 0.0;
            }
        }
        Side::Left => {
            dynamics[ai].rect.x += contact.mtv.1 / 2;
            dynamics[bi].rect.x -= contact.mtv.1 / 2;
            if dynamics[ai].vx < 0.0 {
                dynamics[ai].vx = 0.0;
            }
            if dynamics[bi].vx > 0.0 {
                dynamics[bi].vx = 0.0;
            }
        }
        Side::Right => {
            dynamics[ai].rect.x -= contact.mtv.1 / 2;
            dynamics[bi].rect.x += contact.mtv.1 / 2;
            if dynamics[ai].vx < 0.0 {
                dynamics[ai].vx = 0.0;
            }
            if dynamics[bi].vx > 0.0 {
                dynamics[bi].vx = 0.0;
            }
        }
    }
}
​
fn restitute_ds(ai: usize, dynamics: &mut [Mobile], contact: &mut Contact) {
    match contact.side_a {
        Side::Top => {
            dynamics[ai].rect.y += contact.mtv.1;
            // if displacing opposite to velocity, set y velocity to 0
            if dynamics[ai].vy < 0.0 {
                dynamics[ai].vy = 0.0;
            }
        }
        Side::Bottom => {
            dynamics[ai].rect.y -= contact.mtv.1;
            if dynamics[ai].vy > 0.0 {
                dynamics[ai].vy = 0.0;
            }
        }
        Side::Left => {
            dynamics[ai].rect.x += contact.mtv.1;
            if dynamics[ai].vx < 0.0 {
                dynamics[ai].vx = 0.0;
            }
        }
        Side::Right => {
            dynamics[ai].rect.x -= contact.mtv.1;
            if dynamics[ai].vx < 0.0 {
                dynamics[ai].vx = 0.0;
            }
        }
    }
}
​
fn restitute(statics: &[Wall], dynamics: &mut [Mobile], contacts: &mut [Contact]) {
    // handle restitution of dynamics against dynamics and dynamics against statics wrt contacts.
    // You could instead make contacts `Vec<Contact>` if you think you might remove contacts.
    // You could also add an additional parameter, a slice or vec representing how far we've displaced each dynamic, to avoid allocations if you track a vec of how far things have been moved.
    // You might also want to pass in another &mut Vec<Contact> to be filled in with "real" touches that actually happened.
    contacts.sort_unstable_by_key(|c| -(c.mtv.0 * c.mtv.0 + c.mtv.1 * c.mtv.1));
    // Keep going!  Note that you can assume every contact has a dynamic object in .a.
    // You might decide to tweak the interface of this function to separately take dynamic-static and dynamic-dynamic contacts, to avoid a branch inside of the response calculation.
    // Or, you might decide to calculate signed mtvs taking direction into account instead of the unsigned displacements from rect_displacement up above.  Or calculate one MTV per involved entity, then apply displacements to both objects during restitution (sorting by the max or the sum of their magnitudes)
    let mut restituted: HashMap<ColliderID, Mobile> = HashMap::new();
    for contact in contacts {
        if let ColliderID::Dynamic(ai) = contact.a {
            match contact.b {
                ColliderID::Dynamic(bi) => {
                    if !restituted.contains_key(&contact.a) {
                        restitute_dd(ai, bi, dynamics, contact);
                        restituted.insert(contact.a, dynamics[ai]);
                        restituted.insert(contact.b, dynamics[bi]);
                    } else if let Some(disp) =
                        rect_displacement(dynamics[ai].rect, dynamics[bi].rect)
                    {
                        contact.mtv = (disp.0, disp.1);
                        contact.side_a = disp.2;
                        restitute_dd(ai, bi, dynamics, contact);
                    }
                }
                ColliderID::Static(bi) => {
                    if !restituted.contains_key(&contact.a) {
                        restitute_ds(ai, dynamics, contact);
                        restituted.insert(contact.a, dynamics[ai]);
                    } else if let Some(disp) =
                        rect_displacement(dynamics[ai].rect, statics[bi].rect)
                    {
                        contact.mtv = (disp.0, disp.1);
                        contact.side_a = disp.2;
                        restitute_ds(ai, dynamics, contact);
                    }
                }
            }
        }
    }
}
​
/***
fn main() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = PhysicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Collision2D")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_resizable(false)
            .build(&event_loop)
            .unwrap()
    };
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap()
    };
    let mut player = Mobile {
        rect: Rect {
            x: 32,
            y: HEIGHT as i32 - 16 - 8,
            w: 8,
            h: 8,
        },
        vx: 0.0,
        vy: 0.0,
    };
    let walls = [
        Wall {
            rect: Rect {
                x: 0,
                y: 0,
                w: WIDTH as u16,
                h: 16,
            },
        },
        Wall {
            rect: Rect {
                x: 0,
                y: 0,
                w: 16,
                h: HEIGHT as u16,
            },
        },
        Wall {
            rect: Rect {
                x: WIDTH as i32 - 16,
                y: 0,
                w: 16,
                h: HEIGHT as u16,
            },
        },
        Wall {
            rect: Rect {
                x: 0,
                y: HEIGHT as i32 - 16,
                w: WIDTH as u16,
                h: 16,
            },
        },
        Wall {
            rect: Rect {
                x: WIDTH as i32 / 2 - 16,
                y: HEIGHT as i32 / 2 - 16,
                w: 32,
                h: 32,
            },
        },
    ];
    // How many frames have we simulated?
    let mut frame_count: usize = 0;
    // How many unsimulated frames have we saved up?
    let mut available_time = 0.0;
    // Track beginning of play
    let start = Instant::now();
    let mut contacts = vec![];
    let mut mobiles = [player];
    // Track end of the last frame
    let mut since = Instant::now();
    // Track acceleration
    let acc = 0.01;
    let mut ax = 0.0;
    let mut ay = 0.0;
    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            let fb = pixels.get_frame();
            clear(fb, CLEAR_COL);
            // Draw the walls
            for w in walls.iter() {
                rect(fb, w.rect, WALL_COL);
            }
            // Draw the player
            rect(fb, mobiles[0].rect, PLAYER_COL);
            // Flip buffers
            if pixels.render().is_err() {
                *control_flow = ControlFlow::Exit;
                return;
            }
​
            // Rendering has used up some time.
            // The renderer "produces" time...
            available_time += since.elapsed().as_secs_f64();
        }
        // Handle input events
        if input.update(event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            // Resize the window if needed
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }
        }
        // And the simulation "consumes" it
        while available_time >= DT {
            let player = &mut mobiles[0];
            // Eat up one frame worth of time
            available_time -= DT;
​
            let now = Instant::now();
            let dt = (now - since).as_secs_f32();
​
            // Player control goes here; determine player acceleration
            if input.key_held(VirtualKeyCode::Right) {
                ax += acc;
            }
​
            if input.key_held(VirtualKeyCode::Left) {
                ax -= acc;
            }
​
            if input.key_held(VirtualKeyCode::Up) {
                ay -= acc;
            }
​
            if input.key_held(VirtualKeyCode::Down) {
                ay += acc;
            }
​
            if !input.key_pressed(VirtualKeyCode::Down) && !input.key_pressed(VirtualKeyCode::Up) {
                ay -= signum(player.vy) * acc;
            }
​
            if !input.key_pressed(VirtualKeyCode::Left) && !input.key_pressed(VirtualKeyCode::Right)
            {
                ax -= signum(player.vx) * acc;
            }
​
            // Determine player velocity
            player.vx += ax * dt;
            player.vy += ay * dt;
​
            // Update player position
            player.rect.x += player.vx as i32;
            player.rect.y += player.vy as i32;
​
            // Detect collisions: Generate contacts
            contacts.clear();
            gather_contacts(&walls, &mobiles, &mut contacts);
​
            // Handle collisions: Apply restitution impulses.
            restitute(&walls, &mut mobiles, &mut contacts);
​
            // Update game rules: What happens when the player touches things?
​
            // Increment the frame counter
            frame_count += 1;
        }
        // Request redraw
        window.request_redraw();
        // When did the last frame end?
        since = Instant::now();
    });
}
*/


