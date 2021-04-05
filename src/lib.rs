use pixels::{Pixels, SurfaceTexture};
use rand::distributions::{Bernoulli, Distribution};
use std::collections::HashSet;
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
// Then we can use as usual.  The screen module will have drawing utilities.
pub mod screen;
use crate::screen::*;
// Lazy glob imports
pub mod collision;
use crate::collision::*;
// Texture has our image loading and processing stuff
pub mod texture;
use crate::texture::*;
pub mod animation;
use crate::animation::*;
pub mod audio;
pub mod background;
use crate::background::*;
pub mod obstacle;
use crate::obstacle::*;
pub mod scores;
pub mod sprite;
use crate::sprite::*;
pub mod text;
use crate::text::*;
pub mod tiles;
pub use crate::tiles::*;
pub mod types;
use crate::types::*;

const DEPTH: usize = 4;
const DT: f64 = 1.0 / 60.0;
