use anim2d::collision::*;
use anim2d::scores::Score;
use anim2d::scores::Scores;
use pixels::{Pixels, SurfaceTexture};
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use anim2d::animation::*;
use anim2d::background::*;
use anim2d::screen::Screen;
use anim2d::sprite::*;
use anim2d::text::*;
use anim2d::texture::Texture;
use anim2d::tiles::Tilemap;
use anim2d::tiles::*;
use anim2d::types::Vec2f;
use anim2d::types::*;

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
    bg_tilemaps: Vec<Rc<RefCell<Tilemap>>>,
    camera_position: Vec2f,
    camera_speed: f32,
    mode: Mode,
    font: Rc<Font>,
    level: usize,
    text: Vec<Text>,
    scores: Scores,
    start: Instant,
    og_tilemaps: Vec<Tilemap>,
}

// seconds per frame
const DT: f64 = 1.0 / 60.0;

const WIDTH: usize = 512;
const HEIGHT: usize = 1024;
const DEPTH: usize = 4;
const PLAYER_WIDTH: u16 = 64;
const PLAYER_HEIGHT: u16 = 64;
const START_SPEED: f32 = 0.5;
const SPRITE_INITIAL_X: f32 = 60.0;
const SPRITE_INITIAL_Y: f32 = 112.0;

fn main() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Dig to Free the Beached Whale")
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
    let scuba = Rc::new(Texture::with_file(Path::new("content/scubasprite.png")));
    let tex = Rc::new(Texture::with_file(Path::new("content/tiles_dig.png")));
    let start = Background::new(
        &Rc::new(Texture::with_file(Path::new("content/startscreen.png"))),
        WIDTH,
        HEIGHT,
    );
    let end = Background::new(
        &Rc::new(Texture::with_file(Path::new("content/endscreen.png"))),
        WIDTH,
        HEIGHT,
    );
    let tileset = Rc::new(Tileset::new(
        vec![
            Tile {
                solid: false,
                explode: false,
                destructible: true,
            }, // dirt
            Tile {
                solid: true,
                explode: false,
                destructible: true,
            }, // rock
            Tile {
                solid: true,
                explode: false,
                destructible: true,
            }, // skull
            Tile {
                solid: false,
                explode: false,
                destructible: true,
            }, // dynamite
            Tile {
                solid: false,
                explode: false,
                destructible: true,
            }, // water
            Tile {
                solid: true,
                explode: false,
                destructible: false,
            }, // walls
        ],
        &tex,
    ));
    let map1 = Tilemap::new(
        Vec2f(0.0, 0.0),
        (16, 32),
        &tileset,
        vec![
            5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 1, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0,
            0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 1, 0, 0, 5, 5, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0,
            0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 1,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 1, 0, 0,
            0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 5, 5, 0, 1, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5,
            5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 5, 5, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5,
        ],
    );

    let map2 = Tilemap::new(
        Vec2f(0.0, 1024.0),
        (16, 32),
        &tileset,
        vec![
            5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0,
            0, 0, 5, 5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 1, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0,
            0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 5, 5, 0, 0, 0, 1, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 1, 0, 0, 5, 5, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0,
            0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 1,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 5, 5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0,
            5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 2, 0, 0, 2, 0, 0, 0, 0,
            0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 1, 0, 0,
            0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 5, 5, 0, 1, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 1, 0, 0, 5, 5, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0,
            0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 5, 5, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5,
            5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 5, 5, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5,
        ],
    );

    let map3 = Tilemap::new(
        Vec2f(0.0, 2048.0),
        (16, 32),
        &tileset,
        vec![
            5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 5, 5, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 2, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 1, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0,
            0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 5, 5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0,
            0, 1, 0, 0, 5, 5, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 2, 0, 5, 5, 0, 0, 0, 0,
            0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 1,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 5, 5, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 5, 5, 0, 2, 0, 0, 0, 1, 0, 0,
            0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 5, 5, 0, 1, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 5, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5,
            5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 5, 5, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 5,
        ],
    );
    let map4 = Tilemap::new(
        Vec2f(0.0, 3072.0),
        (16, 32),
        &tileset,
        vec![
            5, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 1, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 1, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0,
            0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 5, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 1, 0, 0, 5, 5, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0,
            0, 0, 1, 0, 0, 0, 0, 0, 2, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 1,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 5, 5, 0, 1, 0, 2, 0, 0, 0, 0, 0, 0, 1,
            0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 1, 0, 0,
            0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 5, 5, 0, 1, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0,
            0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5,
            5, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 1, 5, 5, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 5,
        ],
    );

    let swim_frames = Rectf::create_frames(0, 4, PLAYER_WIDTH, PLAYER_HEIGHT);
    let swim_timing = vec![6, 6, 6, 6];

    let swim = Rc::new(Animation::new(swim_frames, swim_timing, true));
    let animations = vec![swim];
    let animations_clone = animations.clone();
    let player = Sprite::new(
        &scuba,
        &animations_clone[0],
        AnimationState::new(),
        Rectf {
            x: 60.0,
            y: 112.0,
            w: PLAYER_WIDTH,
            h: PLAYER_HEIGHT,
        },
        0.0,
        0.0,
    );
    let sprites = vec![player];

    let start = Background::new(
        &Rc::new(Texture::with_file(Path::new("content/startdig.png"))),
        WIDTH,
        HEIGHT,
    );
    let end = Background::new(
        &Rc::new(Texture::with_file(Path::new("content/enddig.png"))),
        WIDTH,
        HEIGHT,
    );

    let font = Rc::new(Font {
        image: Rc::new(Texture::with_file(Path::new("content/ascii.png"))),
    });

    let mut scores = Scores::new("data/scores.json");
    scores.sort();

    let original_map1 = map1.clone();
    let original_map2 = map2.clone();
    let original_map3 = map3.clone();
    let original_map4 = map4.clone();
    // Track beginning of play
    let start_time = Instant::now();
    let mut state = GameState {
        // initial game state...
        animations,
        sprites,
        textures: vec![scuba],
        backgrounds: vec![start, end],
        curr_location: 0,
        bg_tilemaps: vec![
            Rc::new(RefCell::new(map1)),
            Rc::new(RefCell::new(map2)),
            Rc::new(RefCell::new(map3)),
            Rc::new(RefCell::new(map4)),
        ],
        camera_position: Vec2f(0.0, 0.0),
        camera_speed: 0.0,
        mode: Mode::TitleScreen,
        font,
        level: 0,
        text: vec![],
        scores,
        start: start_time,
        og_tilemaps: vec![original_map1, original_map2, original_map3, original_map4],
    };

    // How many frames have we simulated?
    let mut frame_count: usize = 0;
    // How many unsimulated frames have we saved up?
    let mut available_time = 0.0;
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

            update_game(&mut state, &input);

            // Increment the frame counter
            frame_count += 1;
        }
        // Request redraw
        window.request_redraw();
        // When did the last frame end?
        since = Instant::now();
    });
}

fn draw_scores(state: &mut GameState, screen: &mut Screen) {
    // scores box
    let box_color = Rgba(115, 115, 115, 0);
    let r = Rect {
        x: 120,
        y: 475,
        w: 270,
        h: 275,
    };
    screen.rect(r, box_color);
    screen.line(
        Vec2f(r.x as f32, r.y as f32 - 5.0),
        Vec2f(r.x as f32 + r.w as f32, r.y as f32 - 5.0),
        box_color,
    );
    screen.line(
        Vec2f(r.x as f32, (r.y + r.h as i32) as f32 + 5.0),
        Vec2f(r.x as f32 + r.w as f32, (r.y + r.h as i32) as f32 + 5.0),
        box_color,
    );

    // scores text
    let mut scores_text = Text::new(
        state.font.clone(),
        "HIGH SCORES",
        Vec2f(r.x as f32 + 40.0, r.y as f32 + 30.0),
    );
    screen.draw_text(&mut scores_text);

    for (i, score) in state.scores.scores.iter().enumerate() {
        let mut score_text = Text::new(
            state.font.clone(),
            format!("{}. {} seconds", i + 1, score.value).as_str(),
            Vec2f(r.x as f32 + 20.0, r.y as f32 + 40.0 + 35.0 * (i + 1) as f32),
        );
        screen.draw_text(&mut score_text);
    }
}

fn draw_game(state: &mut GameState, screen: &mut Screen) {
    // Call screen's drawing methods to render the game state
    screen.clear(Rgba(80, 80, 80, 255));

    match state.mode {
        Mode::TitleScreen => {
            screen.draw_background(&state.backgrounds[0]);
            draw_scores(state, screen);

            // start text
            let mut start_text = Text::new(
                state.font.clone(),
                "Press enter to start",
                Vec2f(100.0, 850.0),
            );

            screen.draw_text(&mut start_text);
        }
        Mode::GamePlay => {
            let screen_corners = vec![
                Vec2f(state.camera_position.0, state.camera_position.1),
                Vec2f(
                    state.camera_position.0 + WIDTH as f32,
                    state.camera_position.1,
                ),
                Vec2f(
                    state.camera_position.0,
                    state.camera_position.1 + HEIGHT as f32,
                ),
                Vec2f(
                    state.camera_position.0 + WIDTH as f32,
                    state.camera_position.1 + HEIGHT as f32,
                ),
            ];
            let mut draw_bgmaps = vec![];
            for posn in screen_corners {
                if let Some(i) = tile_map_at(posn, &state.bg_tilemaps) {
                    let map = &state.bg_tilemaps[i];
                    if !draw_bgmaps.contains(&map) {
                        draw_bgmaps.push(map);
                    }
                }
            }

            for map in draw_bgmaps {
                let map = map.borrow();
                map.draw(screen);
            }

            for s in state.sprites.iter() {
                screen.draw_sprite(s);
            }

            // start text
            let mut time = Text::new(
                state.font.clone(),
                format!("TIME: {}", state.start.elapsed().as_secs()).as_str(),
                Vec2f(
                    40.0 + state.camera_position.0,
                    60.0 + state.camera_position.1,
                ),
            );

            screen.draw_text(&mut time);
        }
        Mode::EndGame => {
            screen.draw_background(&state.backgrounds[1]);
            let mut game_over = Text::new(state.font.clone(), "GAME OVER", Vec2f(175.0, 250.0));

            draw_scores(state, screen);

            let mut try_again = Text::new(
                state.font.clone(),
                "Press enter to play again",
                Vec2f(70.0, 850.0),
            );

            screen.draw_text(&mut game_over);
            screen.draw_text(&mut try_again);
        }
    }
}

fn update_game(state: &mut GameState, input: &WinitInputHelper) {
    match state.mode {
        Mode::TitleScreen => {
            if input.key_held(VirtualKeyCode::Return) {
                state.start = Instant::now();
                state.mode = Mode::GamePlay;
            }
        }
        Mode::GamePlay => {
            if !&state.sprites[0].on_screen(state.camera_position, HEIGHT, WIDTH) {
                state.mode = Mode::EndGame;
            };

            tile_collision(state);

            // change x position
            if input.key_pressed(VirtualKeyCode::Left) {
                state.sprites[0].rect.x = (state.sprites[0].rect.x - 2.0).max(32.0);
            }
            if input.key_pressed(VirtualKeyCode::Right) {
                state.sprites[0].rect.x = (state.sprites[0].rect.x + 2.0).min(397.5);
            }
            if input.key_pressed(VirtualKeyCode::Down) {
                state.sprites[0].rect.y += 2.0;
            }
            if input.key_pressed(VirtualKeyCode::Up) {
                state.sprites[0].rect.y -= 2.0;
            }

            // reached bottom of game
            if state.sprites[0].rect.y > 4096.0 {
                let time = state.start.elapsed().as_secs() as i16;
                let score = Score { value: time };
                state.scores.scores.push(score);
                state.scores.sort();
                state.scores.save("data/scores.json");
                state.mode = Mode::EndGame;
            }

            state.sprites[0].tick_forward();

            update_camera(state);
        }
        Mode::EndGame => {
            state.camera_position = Vec2f(0.0, 0.0);
            state.camera_speed = START_SPEED;
            state.level = 0;
            state.sprites[0].rect.x = SPRITE_INITIAL_X;
            state.sprites[0].rect.y = SPRITE_INITIAL_Y;

            if input.key_held(VirtualKeyCode::Return) {
                state.start = Instant::now();
                let mut bg_tilemaps = vec![];
                for map in &state.og_tilemaps {
                    bg_tilemaps.push(Rc::new(RefCell::new(map.clone())));
                }
                state.bg_tilemaps = bg_tilemaps;
                state.mode = Mode::GamePlay;
            }
        }
    }
}

fn tile_collision(state: &mut GameState) {
    let x = state.sprites[0].rect.x;
    let y = state.sprites[0].rect.y;

    let tl = Vec2f(x + 18.0, y);
    let tr = Vec2f(x + state.sprites[0].rect.w as f32 - 18.0, y);
    let bl = Vec2f(x + 18.0, y + state.sprites[0].rect.h as f32);
    let bm = Vec2f(
        x + (state.sprites[0].rect.w) as f32 / 2.0,
        y + state.sprites[0].rect.h as f32,
    );
    let br = Vec2f(
        x + state.sprites[0].rect.w as f32 - 10.0,
        y + state.sprites[0].rect.h as f32,
    );
    let ml = Vec2f(
        x + state.sprites[0].rect.w as f32 + 18.0,
        (y + state.sprites[0].rect.h as f32) as f32 / 2.0,
    );
    let mr = Vec2f(
        x + (state.sprites[0].rect.w) as f32 / 2.0 - 18.0,
        y + (state.sprites[0].rect.h as f32) / 2.0,
    );
    let posns = vec![tl, tr, bl, bm, br, ml, mr];

    for (j, posn) in posns.iter().enumerate() {
        let map_idx = tile_map_at(*posn, &state.bg_tilemaps);
        if let Some(i) = map_idx {
            let mut map = state.bg_tilemaps[i].borrow_mut();
            if let Some(t) = map.tile_at(*posn) {
                if t.solid {
                    if *posn == tl || *posn == ml {
                        state.sprites[0].rect.x += 2.0;
                    } else if *posn == tr || *posn == mr {
                        state.sprites[0].rect.x -= 2.0;
                    } else if *posn == bl || *posn == bm || *posn == br {
                        state.sprites[0].rect.y -= 2.0;
                    }
                } else if t.explode {
                    let tindex = map.tile_index(*posn);
                    map.explode_tiles(tindex, TileID(4), *posn);
                } else if (j == 2 || j == 3 || j == 4) && !t.solid {
                    let tindex = map.tile_index(*posn);
                    map.replace_tile(tindex, TileID(4));
                }
            }
        }
    }
}

fn update_camera(state: &mut GameState) {
    // bottom
    if state.sprites[0].rect.y + PLAYER_HEIGHT as f32
        >= state.camera_position.1 + HEIGHT as f32 - 700.0
    {
        state.camera_position.1 += 5.0;
    }
}

fn tile_map_at(posn: Vec2f, tilemaps: &Vec<Rc<RefCell<Tilemap>>>) -> Option<usize> {
    for (i, map) in tilemaps.iter().enumerate() {
        let map = map.borrow();
        let is_on_x = posn.0 >= map.position.0
            && posn.0 <= map.position.0 + (map.size().0 * anim2d::TILE_SZ) as f32;
        let is_on_y = posn.1 >= map.position.1
            && posn.1 <= map.position.1 + (map.size().1 * anim2d::TILE_SZ) as f32;
        if is_on_x && is_on_y {
            return Some(i);
        }
    }
    None
}
