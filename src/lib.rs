// Whoa what's this?
// Mod without brackets looks for a nearby file.
// Then we can use as usual.  The screen module will have drawing utilities.
pub mod screen;
use crate::screen::*;
// Lazy glob imports
pub mod collision;
// Texture has our image loading and processing stuff
pub mod texture;
use crate::texture::*;
pub mod animation;
pub mod audio;
pub mod background;
pub mod obstacle;
pub mod scores;
pub mod sprite;
use crate::sprite::*;
pub mod text;
pub mod tiles;
pub use crate::tiles::*;
pub mod types;
use crate::types::*;
