use crate::animation::AnimationState;
use crate::collision::Contact;
use crate::collision::Mobile;
use pixels::{Pixels, SurfaceTexture};
use rand::distributions::{Bernoulli, Distribution};
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
mod collision;
// Lazy glob imports
use collision::*;
// Texture has our image loading and processing stuff
mod texture;
use texture::Texture;
// Animation will define our animation datatypes and blending or whatever
mod animation;
use animation::Animation;

mod audio;
// Sprite will define our movable sprites
mod sprite;
// Lazy glob import, see the extension trait business later for why
use sprite::*;
// And we'll put our general purpose types like color and geometry here:
mod types;
use types::*;

mod text;
use text::*;

mod background;
use background::*;

mod obstacle;
use obstacle::*;

mod tiles;
use tiles::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Mode {
    TitleScreen,
    GamePlay,
    EndGame,
}

// Now this main module is just for the run-loop and rules processing.
struct GameState {
    // What data do we need for this game?  Wall positions?
    // Colliders?  Sprites and stuff?
    animations: Vec<Rc<Animation>>,
    textures: Vec<Rc<Texture>>,
    sprites: Vec<Sprite>,
    backgrounds: Vec<Background>,
    curr_location: usize,
    obstacles: Vec<Obstacle>,
    tilemaps: Vec<Rc<Tilemap>>,
    camera_position: Vec2f,
    camera_speed: f32,
    mode: Mode,
    font: Font, // right_bound: usize,
                // left_bound: usize,
                // top_bound: usize,
                // bottom_bound: usize
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

const WIDTH: usize = 512;
const HEIGHT: usize = 256;
const DEPTH: usize = 4;
const PLAYER_WIDTH: u16 = 32;
const PLAYER_HEIGHT: u16 = 32;
const FONT_SIZE: f32 = 20.0;

const CLEAR_COL: Rgba = Rgba(32, 32, 64, 255);
const WALL_COL: Rgba = Rgba(200, 200, 200, 255);
const PLAYER_COL: Rgba = Rgba(255, 128, 128, 255);

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
    let astronaut = Rc::new(Texture::with_file(Path::new("content/Astronaut-Sheet.png")));
    let tex = Rc::new(Texture::with_file(Path::new("content/spacetiles.png")));
    let tileset = Rc::new(Tileset::new(
        vec![
            Tile { solid: false }, // dark
            Tile { solid: false }, // star
            Tile { solid: false }, // moon
            Tile { solid: false }, // planet TL
            Tile { solid: false }, // planet TR
            Tile { solid: false }, // planet BL
            Tile { solid: false }, // planet BR
            Tile { solid: true },  // meteor
            Tile { solid: false }, // transparent
        ],
        &tex,
    ));
    let map1 = Tilemap::new(
        Vec2f(0.0, 0.0),
        (16, 8),
        &tileset,
        vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 3, 4,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
        ],
    );

    let map2 = Tilemap::new(
        Vec2f(512.0, 0.0),
        (16, 8),
        &tileset,
        vec![
            0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 1, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            5, 6, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0,
        ],
    );

    let map3 = Tilemap::new(
        Vec2f(1024.0, 0.0),
        (16, 8),
        &tileset,
        vec![
            1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 3,
            4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
        ],
    );
    let map4 = Tilemap::new(
        Vec2f(1536.0, 0.0),
        (16, 8),
        &tileset,
        vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0,
            0, 0, 0, 1, 0, 0, 0, 0, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
    );

    let meteors = Tilemap::new(
        Vec2f(512.0, 0.0),
        (64, 8),
        &tileset,
        Tilemap::generate_rand_map_2(0.95, Vec2f(512.0, 0.0), (64, 8), TileID(8), TileID(7)),
    );

    let space = Background::new(
        &Rc::new(Texture::with_file(Path::new("content/space.png"))),
        WIDTH,
        HEIGHT,
    );

    let walk_frames = Rectf::create_frames(3, 7, PLAYER_WIDTH, PLAYER_HEIGHT);
    let walk_timing = vec![5, 5, 5, 5];

    let walk = Rc::new(Animation::new(walk_frames, walk_timing, true));
    let animations = vec![walk];
    let animations_clone = animations.clone();
    let player = Sprite::new(
        &astronaut,
        &animations_clone[0],
        AnimationState::new(),
        Rectf {
            x: 60.0,
            y: 112.0,
            w: PLAYER_WIDTH,
            h: PLAYER_HEIGHT,
        },
        0.5,
        0.0,
    );
    // let player_clone = player.clone();
    // let player_x = player.position.0;
    // let player_y = player.position.1;
    let ground = Obstacle {
        image: None,
        frame: None,
        tile_id: None,
        rect: Some(Rect {
            x: 0,
            y: 200,
            h: 56,
            w: 2048,
        }),
        destroyed: false,
    };
    let sprites = vec![player];

    let font = Font {
        image: Rc::new(Texture::with_file(Path::new("content/ascii.png"))),
    };

    let mut state = GameState {
        // initial game state...
        animations,
        sprites,
        textures: vec![astronaut],
        // backgrounds: vec![land, space],
        backgrounds: vec![space],
        curr_location: 0,
        obstacles: vec![],
        tilemaps: vec![
            Rc::new(map1),
            Rc::new(map2),
            Rc::new(map3),
            Rc::new(map4),
            Rc::new(meteors),
        ],
        camera_position: Vec2f(0.0, 0.0),
        camera_speed: 0.5,
        mode: Mode::GamePlay,
        font,
    };

    let mut contacts: Vec<Contact> = vec![];
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

            update_game(&mut state, &mut contacts, &input, frame_count);

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

    match state.mode {
        Mode::TitleScreen => screen.bitblt(
            &state.textures[2],
            Rect {
                x: 0,
                y: 0,
                w: 512,
                h: 512,
            },
            Vec2f(0.0, 0.0),
        ),
        Mode::GamePlay => {
            // levels[state.level].0.draw(screen);
            // for ((pos, tex), anim) in state
            //     .positions
            //     .iter()
            //     .zip(state.textures.iter())
            //     .zip(state.anim_state.iter())
            // {
            //     screen.bitblt(tex, anim.frame(), *pos);
            // }
            // screen.draw_background(&state.backgrounds[state.curr_location]);
            for map in tile_map_at(state, screen) {
                map.draw(screen)
            }
            for s in state.sprites.iter() {
                screen.draw_sprite(s);
            }
            for o in state.obstacles.iter() {
                screen.draw_obstacle(o);
            }
            screen.draw_text(&state.font, "our game", Vec2f(100.0, 100.0));
        }
        Mode::EndGame => screen.bitblt(
            &state.textures[3],
            Rect {
                x: 0,
                y: 0,
                w: 512,
                h: 512,
            },
            Vec2f(0.0, 0.0),
        ),
    }
}

fn update_game(
    state: &mut GameState,
    contacts: &mut Vec<Contact>,
    input: &WinitInputHelper,
    frame: usize,
) {
    match state.mode {
        Mode::TitleScreen => {
            if input.key_held(VirtualKeyCode::Return) {
                state.mode = Mode::GamePlay
            }
        }
        Mode::GamePlay => {
            // Player control goes here
            // if input.key_held(VirtualKeyCode::Right) {
            //     state.sprites[0].rect.x += 2;
            // }
            // if input.key_held(VirtualKeyCode::Left) {
            //     state.sprites[0].rect.x -= 2;
            // }
            if !&state.sprites[0].on_screen(state.camera_position, HEIGHT, WIDTH) {
                state.mode = Mode::EndGame;
            };

            state.sprites[0].rect.x += state.sprites[0].vx;

            // change velocity
            if input.key_pressed(VirtualKeyCode::Up) {
                state.sprites[0].vy -= 0.25;
            }
            if input.key_pressed(VirtualKeyCode::Down) {
                state.sprites[0].vy += 0.25;
            }

            // change y position
            state.sprites[0].rect.y += state.sprites[0].vy;

            state.sprites[0].tick_forward();

            scroll_camera(state);

            // Update player position

            // Detect collisions: Generate contacts
            contacts.clear();
            Collision::gather_contacts(&state.obstacles, &state.sprites, contacts);

            // Handle collisions: Apply restitution impulses.
            Collision::restitute(&state.obstacles, &mut state.sprites, contacts);

            // Update game rules: What happens when the player touches things?
        }
        Mode::EndGame => {
            if input.key_held(VirtualKeyCode::R) {
                //     state.positions[0] = Vec2i(levels[0].1[0].1 * 16, levels[0].1[0].2 * 16);
                //     state.velocities[0] = Vec2i(0, 0);
                state.mode = Mode::GamePlay
            }
        }
    }
}

fn scroll_camera(state: &mut GameState) {
    state.camera_position.0 += state.camera_speed;
}

fn update_camera(state: &mut GameState) {
    // right side
    if state.sprites[0].rect.x + PLAYER_WIDTH as f32 >= state.camera_position.0 + WIDTH as f32 - 5.0
    {
        state.camera_position.0 += state.camera_speed;
    }

    // left side
    if state.sprites[0].rect.x <= state.camera_position.0 + 5.0 {
        state.camera_position.0 -= state.camera_speed;
    }

    // top
    if state.sprites[0].rect.y <= state.camera_position.1 + 5.0 {
        state.camera_position.1 -= state.camera_speed;
    }

    // bottom
    if state.sprites[0].rect.y + PLAYER_HEIGHT as f32
        >= state.camera_position.1 + HEIGHT as f32 - 5.0
    {
        state.camera_position.1 += state.camera_speed;
    }
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
