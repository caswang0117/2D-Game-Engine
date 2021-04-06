use kira::manager::AudioManager;
use kira::manager::AudioManagerSettings;
use kira::sound::SoundSettings;
use pixels::{Pixels, SurfaceTexture};
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use anim2d::animation::*;
use anim2d::audio::*;
use anim2d::background::*;
use anim2d::collision::*;
use anim2d::obstacle::*;
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

struct GameState {
    animations: Vec<Rc<Animation>>,
    textures: Vec<Rc<Texture>>,
    sprites: Vec<Sprite>,
    backgrounds: Vec<Background>,
    curr_location: usize,
    obstacles: Vec<Obstacle>,
    bg_tilemaps: Vec<Rc<Tilemap>>,
    obstacle_tilemaps: Vec<Rc<Tilemap>>,
    camera_position: Vec2f,
    camera_speed: f32,
    mode: Mode,
    font: Rc<Font>,
    level: usize,
    text: Vec<Text>,
    audio: Audio,
}

// seconds per frame
const DT: f64 = 1.0 / 60.0;

const WIDTH: usize = 512;
const HEIGHT: usize = 256;
const DEPTH: usize = 4;
const PLAYER_WIDTH: u16 = 32;
const PLAYER_HEIGHT: u16 = 32;
const FONT_SIZE: f32 = 20.0;
const START_P: f32 = 0.97;
const START_SPEED: f32 = 0.5;
const SPRITE_INITIAL_X: f32 = 60.0;
const SPRITE_INITIAL_Y: f32 = 112.0;
const SPRITE_INITIAL_VX: f32 = 0.5;
const SPRITE_INITIAL_VY: f32 = 0.0;
const LEVEL_WIDTH: usize = 2048;
const METEOR_START: f32 = 1400.0;

fn main() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Race to Save the Beached Whale")
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
            }, // dark
            Tile {
                solid: false,
                explode: false,
                destructible: true,
            }, // star
            Tile {
                solid: false,
                explode: false,
                destructible: true,
            }, // moon
            Tile {
                solid: false,
                explode: false,
                destructible: true,
            }, // planet TL
            Tile {
                solid: false,
                explode: false,
                destructible: true,
            }, // planet TR
            Tile {
                solid: false,
                explode: false,
                destructible: true,
            }, // planet BL
            Tile {
                solid: false,
                explode: false,
                destructible: true,
            }, // planet BR
            Tile {
                solid: true,
                explode: false,
                destructible: true,
            }, // meteor
            Tile {
                solid: false,
                explode: false,
                destructible: true,
            }, // transparent
        ],
        &tex,
    ));
    let map1 = Tilemap::new(
        Vec2f(0.0, 0.0),
        (16, 8),
        &tileset,
        vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0,
            3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
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
    let mut audio_manager = AudioManager::new(AudioManagerSettings::default()).unwrap();
    let startscreen = audio_manager
        .load_sound(
            "content/Startscreen.wav",
            SoundSettings::default(), // SoundSettings::new().semantic_duration(Tempo(128.0).beats_to_seconds(8.0)),
        )
        .unwrap();
    let gameplay = audio_manager
        .load_sound(
            "content/GamePlay.mp3",
            SoundSettings::default(), // SoundSettings::new().semantic_duration(Tempo(128.0).beats_to_seconds(8.0)),
        )
        .unwrap();
    let endscreen = audio_manager
        .load_sound(
            "content/endscreen.mp3",
            SoundSettings::default(), // SoundSettings::new().semantic_duration(Tempo(128.0).beats_to_seconds(8.0)),
        )
        .unwrap();
    let collision = audio_manager
        .load_sound(
            "content/collision.mp3",
            SoundSettings::default(), // SoundSettings::new().semantic_duration(Tempo(128.0).beats_to_seconds(8.0)),
        )
        .unwrap();
    let sound_handles = vec![startscreen, gameplay, collision, endscreen];
    let audio = Audio::new(audio_manager, sound_handles);

    let meteors = Tilemap::new(
        Vec2f(METEOR_START as f32, 0.0),
        (64, 8),
        &tileset,
        Tilemap::generate_rand_map_2(0.95, (64, 8), TileID(8), TileID(7)),
    );

    let meteors2 = Tilemap::new(
        Vec2f(METEOR_START + (meteors.dims.0 * TILE_SZ) as f32, 0.0),
        // Vec2f(3698.0, 0.0),
        (64, 8),
        &tileset,
        Tilemap::generate_rand_map_2(0.95, (64, 8), TileID(8), TileID(7)),
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
        SPRITE_INITIAL_VX,
        0.0,
    );
    // let player_clone = player.clone();
    // let player_x = player.position.0;
    // let player_y = player.position.1;
    let sprites = vec![player];

    let font = Rc::new(Font {
        image: Rc::new(Texture::with_file(Path::new("content/ascii.png"))),
    });
    let text1 = Text::new(Rc::clone(&font), "It is March 25, 2021.", Vec2f(75.0, 50.0));

    let text2 = Text::new(
        Rc::clone(&font),
        "The Ever Given is still",
        Vec2f(text1.pos.0 + text1.length as f32 - 80.0, 150.0),
    );
    let text3 = Text::new(
        Rc::clone(&font),
        "stuck in the Suez Canal.",
        Vec2f(text1.pos.0 + text1.length as f32 - 50.0, 170.0),
    );

    let text4 = Text::new(
        Rc::clone(&font),
        "You were sent to",
        Vec2f(text3.pos.0 + text3.length as f32 + 50.0, 70.0),
    );
    let text5 = Text::new(
        Rc::clone(&font),
        "get help from the aliens",
        Vec2f(text3.pos.0 + text3.length as f32 + 70.0, 90.0),
    );
    let text6 = Text::new(
        Rc::clone(&font),
        "to get this beached whale",
        Vec2f(text5.pos.0 + text5.length as f32 - 215.0, 150.0),
    );
    let text7 = Text::new(
        Rc::clone(&font),
        "back to the waters.",
        Vec2f(text5.pos.0 + text5.length as f32 - 185.0, 170.0),
    );

    let text8 = Text::new(
        Rc::clone(&font),
        "Move up and down",
        Vec2f(text7.pos.0 + text7.length as f32 + 170.0, 180.0),
    );
    let text9 = Text::new(
        Rc::clone(&font),
        "to avoid the meteors",
        Vec2f(text7.pos.0 + text7.length as f32 + 190.0, 200.0),
    );
    let text10 = Text::new(
        Rc::clone(&font),
        "and get back to Earth!",
        Vec2f(text7.pos.0 + text7.length as f32 + 210.0, 220.0),
    );

    let text11 = Text::new(
        Rc::clone(&font),
        "Good luck, humble servant.",
        Vec2f(text10.pos.0 + text10.length as f32 + 50.0, 80.0),
    );

    let text12 = Text::new(
        Rc::clone(&font),
        "LEVEL 0",
        Vec2f(
            text11.pos.0 + text11.length as f32 + 50.0,
            (HEIGHT / 2) as f32,
        ),
    );

    let display_text = vec![
        text1, text2, text3, text4, text5, text6, text7, text8, text9, text10, text11, text12,
    ];

    let mut state = GameState {
        // initial game state...
        animations,
        sprites,
        textures: vec![astronaut],
        backgrounds: vec![start, end],
        curr_location: 0,
        obstacles: vec![],
        bg_tilemaps: vec![Rc::new(map1), Rc::new(map2), Rc::new(map3), Rc::new(map4)],
        obstacle_tilemaps: vec![Rc::new(meteors), Rc::new(meteors2)],
        camera_position: Vec2f(0.0, 0.0),
        camera_speed: START_SPEED,
        mode: Mode::TitleScreen,
        font,
        level: 0,
        text: display_text,
        audio,
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
        Mode::TitleScreen => {
            screen.draw_background(&state.backgrounds[0]);
            let mut start_text = Text::new(
                state.font.clone(),
                "Press enter to start",
                Vec2f(100.0, 200.0),
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
            let mut draw_obsmaps = vec![];
            for posn in screen_corners {
                if let Some(i) = tile_map_at(posn, &state.bg_tilemaps) {
                    let map = &state.bg_tilemaps[i];
                    if !draw_bgmaps.contains(&map) {
                        draw_bgmaps.push(map);
                    }
                }
                if let Some(i) = tile_map_at(posn, &state.obstacle_tilemaps) {
                    let map = &state.obstacle_tilemaps[i];
                    if !draw_obsmaps.contains(&map) {
                        draw_obsmaps.push(map);
                    }
                }
            }

            for map in draw_bgmaps {
                map.draw(screen);
            }
            for map in draw_obsmaps {
                map.draw(screen);
            }

            //infinite tilemaps
            update_tilemaps(
                state.camera_position,
                &mut state.bg_tilemaps,
                false,
                state.level,
            );
            update_tilemaps(
                state.camera_position,
                &mut state.obstacle_tilemaps,
                true,
                state.level,
            );

            for s in state.sprites.iter() {
                screen.draw_sprite(s);
            }
            for o in state.obstacles.iter() {
                screen.draw_obstacle(o);
            }

            // Check screen position to update level
            let start_pos = (state.camera_position.0 - METEOR_START as f32).max(0.0);
            if start_pos as usize / LEVEL_WIDTH != state.level {
                new_level(state);
            }

            for text in &mut state.text {
                screen.draw_text(text);
            }
        }
        Mode::EndGame => {
            screen.draw_background(&state.backgrounds[1]);
            let mut game_over = Text::new(state.font.clone(), "GAME OVER", Vec2f(175.0, 90.0));

            let mut try_again = Text::new(
                state.font.clone(),
                "Press enter to play again",
                Vec2f(70.0, 130.0),
            );

            screen.rect(
                Rect {
                    w: 164,
                    h: 30,
                    x: 164,
                    y: 82,
                },
                Rgba(215, 0, 0, 255),
            );
            screen.line(
                Vec2f(160.0, 82.0),
                Vec2f(160.0, 112.0),
                Rgba(215, 0, 0, 255),
            );
            screen.line(
                Vec2f(156.0, 82.0),
                Vec2f(156.0, 112.0),
                Rgba(215, 0, 0, 255),
            );
            screen.line(
                Vec2f(332.0, 82.0),
                Vec2f(332.0, 112.0),
                Rgba(215, 0, 0, 255),
            );
            screen.line(
                Vec2f(336.0, 82.0),
                Vec2f(336.0, 112.0),
                Rgba(215, 0, 0, 255),
            );

            screen.draw_text(&mut game_over);
            screen.draw_text(&mut try_again);
        }
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
            state
                .audio
                .play(SoundID(0), false, None, AlreadyPlayingAction::Nothing);

            if input.key_held(VirtualKeyCode::Return) {
                state.audio.stop(SoundID(0), None);
                state.mode = Mode::GamePlay
            }
        }
        Mode::GamePlay => {
            state
                .audio
                .play(SoundID(1), true, Some(0.0), AlreadyPlayingAction::Nothing);

            if !&state.sprites[0].on_screen(state.camera_position, HEIGHT, WIDTH) {
                state.audio.stop(SoundID(1), None);
                state
                    .audio
                    .play(SoundID(3), true, Some(0.0), AlreadyPlayingAction::Nothing);
                state.mode = Mode::EndGame;
            };

            tile_collision(state);

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
        }
        Mode::EndGame => {
            state.camera_position = Vec2f(0.0, 0.0);
            state.camera_speed = START_SPEED;
            state.level = 0;
            state.sprites[0].vx = SPRITE_INITIAL_VX;
            state.sprites[0].vy = SPRITE_INITIAL_VY;
            state.sprites[0].rect.x = SPRITE_INITIAL_X;
            state.sprites[0].rect.y = SPRITE_INITIAL_Y;

            let mut bg_tilemaps = vec![];
            for (i, map) in state.bg_tilemaps.iter().enumerate() {
                let new = Tilemap {
                    position: Vec2f((i * WIDTH) as f32, 0.0),
                    dims: map.dims,
                    tileset: Rc::clone(&map.tileset),
                    map: map.map.clone(),
                };
                bg_tilemaps.push(Rc::new(new));
            }

            let mut obstacle_tilemaps = vec![];
            for (i, map) in state.obstacle_tilemaps.iter().enumerate() {
                let new = Tilemap::new(
                    Vec2f(
                        METEOR_START + map.dims.0 as f32 * TILE_SZ as f32 * i as f32,
                        0.0,
                    ),
                    map.dims,
                    &Rc::clone(&map.tileset),
                    Tilemap::generate_rand_map_2(START_P, map.dims, TileID(8), TileID(7)),
                );
                obstacle_tilemaps.push(Rc::new(new));
            }

            state.bg_tilemaps = bg_tilemaps;
            state.obstacle_tilemaps = obstacle_tilemaps;

            if input.key_held(VirtualKeyCode::Return) {
                state.audio.stop(SoundID(3), None);
                state.mode = Mode::GamePlay;
            }
        }
    }
}

fn new_level(state: &mut GameState) {
    state.level += 1;
    state.camera_speed += 0.2;
    state.sprites[0].vx += 0.2;
    let levelup = format!("LEVEL {}", state.level);

    state.text.push(Text::new(
        state.font.clone(),
        &levelup,
        Vec2f(
            state.camera_position.0 + (WIDTH / 2) as f32,
            (HEIGHT / 2) as f32,
        ),
    ));
}

fn update_tilemaps(
    camera_position: Vec2f,
    tilemaps: &mut Vec<Rc<Tilemap>>,
    is_obstacle: bool,
    level: usize,
) {
    let p = START_P - 0.03 * level as f32;
    let first = &tilemaps[0];
    if first.position.0 as usize + first.size().0 * TILE_SZ < camera_position.0 as usize {
        let last = tilemaps.last().unwrap();
        let new: Tilemap;
        if is_obstacle {
            new = Tilemap::new(
                Vec2f(last.position.0 + last.size().0 as f32 * TILE_SZ as f32, 0.0),
                first.dims,
                &Rc::clone(&first.tileset),
                Tilemap::generate_rand_map_2(p, first.dims, TileID(8), TileID(7)),
            );
        } else {
            new = Tilemap {
                position: Vec2f(last.position.0 + last.size().0 as f32 * TILE_SZ as f32, 0.0),
                dims: first.dims,
                tileset: Rc::clone(&first.tileset),
                map: first.map.clone(),
            };
        }
        tilemaps.remove(0);
        tilemaps.push(Rc::new(new));
    }
}

fn tile_collision(state: &mut GameState) {
    let x = state.sprites[0].rect.x;
    let y = state.sprites[0].rect.y;

    let tl = Vec2f(x + 12.0, y + 12.0);
    let tr = Vec2f(x + state.sprites[0].rect.w as f32 - 12.0, y + 12.0);
    let bl = Vec2f(x + 12.0, y + state.sprites[0].rect.h as f32);
    let br = Vec2f(
        x + state.sprites[0].rect.w as f32 - 12.0,
        y + state.sprites[0].rect.h as f32,
    );
    let posns = vec![tl, tr, bl, br];

    for posn in posns {
        let map_idx = tile_map_at(posn, &state.obstacle_tilemaps);
        if let Some(i) = map_idx {
            if let Some(t) = state.obstacle_tilemaps[i].tile_at(posn) {
                if t.solid {
                    state
                        .audio
                        .play(SoundID(2), false, None, AlreadyPlayingAction::Nothing);
                    state
                        .audio
                        .play(SoundID(3), true, Some(0.0), AlreadyPlayingAction::Nothing);
                    state.mode = Mode::EndGame
                }
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

fn tile_map_at(posn: Vec2f, tilemaps: &Vec<Rc<Tilemap>>) -> Option<usize> {
    for (i, map) in tilemaps.iter().enumerate() {
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
