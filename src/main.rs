use pixels::{Pixels, SurfaceTexture};
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

// Whoa what's this?
// Mod without brackets looks for a nearby file.
mod screen;
// Then we can use as usual.  The screen module will have drawing utilities.
use screen::Screen;
// Collision will have our collision bodies and contact types
// mod collision;
// Lazy glob imports
// use collision::*;
// Texture has our image loading and processing stuff
mod texture;
use texture::Texture;
// Animation will define our animation datatypes and blending or whatever
mod animation;
use animation::Animation;
// Sprite will define our movable sprites
mod sprite;
// Lazy glob import, see the extension trait business later for why
use sprite::*;
// And we'll put our general purpose types like color and geometry here:
mod types;
use types::*;

mod background;
use background::*;

mod obstacle;
use obstacle::Obstacle;

mod tiles;
use tiles::*;

// Now this main module is just for the run-loop and rules processing.
struct GameState {
    // What data do we need for this game?  Wall positions?
    // Colliders?  Sprites and stuff?
    animations: Vec<Animation>,
    textures: Vec<Rc<Texture>>,
    sprites: Vec<Sprite>,
    backgrounds: Vec<Background>,
    curr_location: usize,
    ground: Rect,
    obstacles: Vec<Rc<Obstacle>>,
    tilemaps: Vec<Rc<Tilemap>>,
    camera_position: Vec2i,
    // right_bound: usize,
    // left_bound: usize,
    // top_bound: usize,
    // bottom_bound: usize,
}

// impl GameState {
//     pub fn new(
//         animations: Vec<Animation>,
//         textures: Vec<Rc<Texture>>,
//         sprites: Vec<Sprite>,
//         backgrounds: Vec<Background>,
//         curr_location: usize,
//         ground: Rect,
//         obstacles: Vec<Rc<Obstacle>>,
//         tilemaps: Vec<Rc<Tilemap>>,
//         camera_position: Vec2i,
//     ) -> Self {
//         let left_bound = tilemaps[0]
//         Self {
//             animations: animations,
//             textures: textures,
//             sprites: sprites,
//             backgrounds: backgrounds,
//             curr_location: curr_location,
//             ground: ground,
//             obstacles: obstacles,
//             tilemaps: tilemaps,
//             camera_position: camera_position,
//             left_bound:
//         }
//     }
// }
// seconds per frame
const DT: f64 = 1.0 / 60.0;

const WIDTH: usize = 128;
const HEIGHT: usize = 128;
const DEPTH: usize = 4;
const PLAYER_WIDTH: u16 = 100;
const PLAYER_HEIGHT: u16 = 100;
const RIGHT_BOUND: usize = 612;
const LEFT_BOUND: usize = 0;
const TOP_BOUND: usize = 0;
const BOTTOM_BOUND: usize = 128;

fn main() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Anim2D")
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
    let person = Rc::new(Texture::with_file(Path::new("content/Person-sprite.png")));
    let tex = Rc::new(Texture::with_file(Path::new("content/tileset.png")));
    let tileset = Rc::new(Tileset::new(
        vec![
            Tile { solid: false }, // blue
            Tile { solid: true },  // cloud
        ],
        &tex,
    ));
    let map1 = Tilemap::new(
        Vec2i(0, 0),
        (8, 8),
        &tileset,
        vec![
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0,
            0, 1, 0, 0, 1, 0,
        ],
    );

    let map2 = Tilemap::new(
        Vec2i(128, 0),
        (8, 8),
        &tileset,
        vec![
            0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0,
            1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 1, 0, 0,
        ],
    );

    let map3 = Tilemap::new(
        Vec2i(256, 0),
        (8, 8),
        &tileset,
        vec![
            0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0,
            0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0,
            0, 1, 1, 0, 0, 1,
        ],
    );
    let map4 = Tilemap::new(
        Vec2i(384, 0),
        (8, 8),
        &tileset,
        vec![
            0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1,
            0, 0, 0, 1, 0, 0,
        ],
    );
    let land = Background::new(
        &Rc::new(Texture::with_file(Path::new("content/land.png"))),
        WIDTH,
        HEIGHT,
    );
    let space = Background::new(
        &Rc::new(Texture::with_file(Path::new("content/space.png"))),
        WIDTH,
        HEIGHT,
    );

    let walk_frames = Rect::create_frames(0, 4, PLAYER_WIDTH, PLAYER_HEIGHT);
    let walk_timing = vec![3, 3, 3, 3];

    let walk: Animation = Animation::new(walk_frames, walk_timing, true);
    let walk_clone: Animation = walk.clone();

    let mut state = GameState {
        // initial game state...
        animations: vec![walk],
        sprites: vec![Sprite::new(
            &person,
            // Rc::new(walk_clone),
            walk_clone,
            Vec2i(0, 0),
        )],
        textures: vec![person],
        backgrounds: vec![land, space],
        curr_location: 0,
        ground: Rect {
            x: 0,
            y: 900,
            h: 100,
            w: 1000,
        },
        obstacles: vec![],
        tilemaps: vec![Rc::new(map1), Rc::new(map2), Rc::new(map3), Rc::new(map4)],
        camera_position: Vec2i(0, 0),
    };
    // How many frames have we simulated?
    let mut frame_count: usize = 0;
    // How many unsimulated frames have we saved up?
    let mut available_time = 0.0;
    // Track beginning of play
    let start = Instant::now();
    // Track end of the last frame
    let mut since = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            let mut screen = Screen::wrap(
                pixels.get_frame(),
                WIDTH,
                HEIGHT,
                DEPTH,
                state.camera_position,
            );
            // let mut screen = Screen::wrap(pixels.get_frame(), WIDTH, HEIGHT, DEPTH);
            screen.clear(Rgba(0, 0, 0, 0));

            draw_game(&mut state, &mut screen);

            // Flip buffers
            if pixels.render().is_err() {
                *control_flow = ControlFlow::Exit;
                return;
            }

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
            // Eat up one frame worth of time
            available_time -= DT;

            update_game(&mut state, &input, frame_count);

            // Increment the frame counter
            frame_count += 1;
        }
        // Request redraw
        window.request_redraw();
        // When did the last frame end?
        since = Instant::now();
    });
}

fn draw_game(state: &mut GameState, screen: &mut Screen) {
    // Call screen's drawing methods to render the game state
    screen.clear(Rgba(80, 80, 80, 255));
    // screen.draw_background(&state.backgrounds[state.curr_location]);
    for map in tile_map_at(state, screen) {
        map.draw(screen)
    }
    let sprite_pos = state.sprites[0].position;
    // state.tilemaps.draw(screen);
    for s in state.sprites.iter() {
        screen.draw_sprite(s);
    }
}

fn update_game(state: &mut GameState, input: &WinitInputHelper, frame: usize) {
    // Player control goes here
    if input.key_held(VirtualKeyCode::Right) {
        state.sprites[0].position.0 += 2;
    }
    if input.key_held(VirtualKeyCode::Left) {
        state.sprites[0].position.0 -= 2;
    }
    if input.key_held(VirtualKeyCode::Up) {
        state.sprites[0].position.1 -= 2;
    }
    if input.key_held(VirtualKeyCode::Down) {
        state.sprites[0].position.1 += 2;
    }

    state.sprites[0].animation.tick_forward();
    state.backgrounds[state.curr_location].tick_right(WIDTH);

    // right side
    if state.sprites[0].position.0 + PLAYER_WIDTH as i32
        >= state.camera_position.0 + WIDTH as i32 - 5
    {
        state.camera_position.0 += 2;
    }

    // left side
    if state.sprites[0].position.0 <= state.camera_position.0 + 5 {
        state.camera_position.0 -= 2;
    }

    // top
    if state.sprites[0].position.1 <= state.camera_position.1 + 5 {
        state.camera_position.1 -= 2;
    }

    // bottom
    if state.sprites[0].position.1 + PLAYER_HEIGHT as i32
        >= state.camera_position.1 + HEIGHT as i32 - 5
    {
        state.camera_position.1 += 2;
    }

    // Update player position

    // Detect collisions: Generate contacts

    // Handle collisions: Apply restitution impulses.

    // Update game rules: What happens when the player touches things?
}

fn tile_map_at(state: &mut GameState, screen: &mut Screen) -> Vec<Rc<Tilemap>> {
    let screen_pos_l = screen.size();
    let screen_pos_r = (
        screen_pos_l.0 + PLAYER_WIDTH as usize,
        screen_pos_l.1 + PLAYER_HEIGHT as usize,
    );

    let mut show_tilemaps = vec![];
    let mut x_pos_so_far = 0;
    for (i, map) in state.tilemaps.iter().enumerate() {
        x_pos_so_far += map.size().0 * TILE_SZ;
        if x_pos_so_far >= screen_pos_l.0 as usize {
            show_tilemaps.push(Rc::clone(map));
            if x_pos_so_far < screen_pos_r.0 as usize && i + 1 < state.tilemaps.len() {
                show_tilemaps.push(Rc::clone(&state.tilemaps[i + 1]))
            }
        }
    }
    show_tilemaps
}
