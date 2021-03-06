use crate::Rect;
use crate::Screen;
use crate::Texture;
use crate::Vec2f;
use rand::distributions::{Bernoulli, Distribution};
use std::rc::Rc;

pub const TILE_SZ: usize = 32;
/// A graphical tile, we'll implement Copy since it's tiny
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Tile {
    pub solid: bool, // ... any extra data like collision flags or other properties
    pub explode: bool,
    pub destructible: bool,
}

/// A set of tiles used in multiple Tilemaps
#[derive(PartialEq, Debug)]
pub struct Tileset {
    // Tile size is a constant, so we can find the tile in the texture using math
    // (assuming the texture is a grid of tiles).
    pub tiles: Vec<Tile>,
    texture: Rc<Texture>,
    // In this design, each tileset is a distinct image.
    // Maybe not always the best choice if there aren't many tiles in a tileset!
}
/// Indices into a Tileset
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TileID(pub usize);

/// Grab a tile with a given ID
impl std::ops::Index<TileID> for Tileset {
    type Output = Tile;
    fn index(&self, id: TileID) -> &Self::Output {
        &self.tiles[id.0]
    }
}
impl Tileset {
    /// Create a new tileset
    pub fn new(tiles: Vec<Tile>, texture: &Rc<Texture>) -> Self {
        Self {
            tiles,
            texture: Rc::clone(texture),
        }
    }
    /// Get the frame rect for a tile ID
    fn get_rect(&self, id: TileID) -> Rect {
        let idx = id.0;
        let (w, _h) = self.texture.size();
        let tw = w / TILE_SZ;
        let row = idx / tw;
        let col = idx - (row * tw);
        Rect {
            x: col as i32 * TILE_SZ as i32,
            y: row as i32 * TILE_SZ as i32,
            w: TILE_SZ as u16,
            h: TILE_SZ as u16,
        }
    }
    /// Does this tileset have a tile for `id`?
    fn contains(&self, id: TileID) -> bool {
        id.0 < self.tiles.len()
    }
}

#[derive(PartialEq, Debug, Clone)]
/// An actual tilemap
pub struct Tilemap {
    /// Whce the tilemap is in space, use your favorite number type here
    pub position: Vec2f,
    /// How big it is
    pub dims: (usize, usize),
    /// Which tileset is used for this tilemap
    pub tileset: Rc<Tileset>,
    /// A row-major grid of tile IDs in tileset
    pub map: Vec<TileID>,
}

impl Tilemap {
    pub fn new(
        position: Vec2f,
        dims: (usize, usize),
        tileset: &Rc<Tileset>,
        map: Vec<usize>,
    ) -> Self {
        assert_eq!(dims.0 * dims.1, map.len(), "Tilemap is the wrong size!");
        assert!(
            map.iter().all(|tid| tileset.contains(TileID(*tid))),
            "Tilemap refers to nonexistent tiles"
        );
        Self {
            position,
            dims,
            tileset: Rc::clone(tileset),
            map: map.into_iter().map(TileID).collect(),
        }
    }

    pub fn generate_rand_map_2(p: f32, dims: (usize, usize), t1: TileID, t2: TileID) -> Vec<usize> {
        let m = Bernoulli::new(p as f64).unwrap();
        let mut map = vec![];

        for _i in 0..dims.0 * dims.1 {
            let v = m.sample(&mut rand::thread_rng());
            if v {
                map.push(t1.0);
            } else {
                map.push(t2.0);
            }
        }
        map
    }

    pub fn tile_id_at(&self, Vec2f(x, y): Vec2f) -> TileID {
        // Translate into map coordinates
        assert!(
            x >= self.position.0 && x <= self.position.0 + (self.dims.0 * TILE_SZ) as f32,
            "Tile X coordinate {} out of bounds {}, {}",
            x,
            self.position.0,
            self.position.0 + (self.dims.0 * TILE_SZ) as f32
        );
        assert!(
            y >= self.position.1 && y <= self.position.1 + (self.dims.1 * TILE_SZ) as f32,
            "Tile Y coordinate {} out of bounds {}, {}",
            y,
            self.position.1,
            self.position.1 + (self.dims.1 * TILE_SZ) as f32
        );
        let x = ((x - self.position.0) / TILE_SZ as f32).floor(); // gets into world coordinates in frame of tile map
        let y = ((y - self.position.1) / TILE_SZ as f32).floor().min(31.0);
        assert!(
            (y as usize * self.dims.0 + x as usize) as usize <= self.map.len() - 1,
            "x coord: {}, y coord: {}",
            x,
            y
        );
        self.map[y as usize * self.dims.0 + x as usize]
    }

    pub fn tile_index(&self, Vec2f(x, y): Vec2f) -> usize {
        assert!(
            // x >= 0.0 && x < self.dims.0 as f32,
            x >= self.position.0 && x <= self.position.0 + (self.dims.0 * TILE_SZ) as f32,
            "Tile X coordinate {} out of bounds {}, {}",
            x,
            self.position.0,
            self.position.0 + (self.dims.0 * TILE_SZ) as f32
        );
        assert!(
            y >= self.position.1 && y <= self.position.1 + (self.dims.1 * TILE_SZ) as f32,
            "Tile Y coordinate {} out of bounds {}, {}",
            y,
            self.position.1,
            self.position.1 + (self.dims.1 * TILE_SZ) as f32
        );
        let x = ((x - self.position.0) / TILE_SZ as f32).floor(); // gets into world coordinates in frame of tile map
        let y = ((y - self.position.1) / TILE_SZ as f32).floor().min(31.0);
        assert!(
            (y as usize * self.dims.0 + x as usize) as usize <= self.map.len() - 1,
            "x coord: {}, y coord: {}",
            x,
            y
        );
        return y as usize * self.dims.0 + x as usize;
    }

    pub fn replace_tile(&mut self, index: usize, tile: TileID) {
        self.map[index] = tile;
    }

    pub fn explode_tiles(&mut self, index: usize, new_tile: TileID, posn: Vec2f) {
        self.map[index] = new_tile;
        let b = Vec2f(posn.0, posn.1 + TILE_SZ as f32 - 0.2);
        let bl = Vec2f(posn.0 - TILE_SZ as f32 + 0.2, posn.1 + TILE_SZ as f32 + 0.2);
        let br = Vec2f(posn.0 + TILE_SZ as f32 - 0.2, posn.1 + TILE_SZ as f32 + 0.2);

        let posns = vec![b, bl, br];
        let mut indices = vec![];
        for posn in posns.clone() {
            indices.push(self.tile_index(posn));
        }

        for (x, i) in indices.iter().enumerate() {
            if i < &self.map.len() {
                let tile = self.tileset[self.map[*i]];
                if tile.destructible {
                    self.map[*i] = new_tile;
                }
                if tile.explode {
                    self.explode_tiles(*i, new_tile, posns[x]);
                }
            }
        }
    }

    pub fn size(&self) -> (usize, usize) {
        self.dims
    }

    pub fn tile_at(&self, posn: Vec2f) -> Option<Tile> {
        if (posn.0 >= self.position.0 && posn.0 < self.position.0 + (self.dims.0 * TILE_SZ) as f32)
            || (posn.1 >= self.position.1
                && posn.1 <= self.position.1 + (self.dims.1 * TILE_SZ) as f32)
        {
            Some(self.tileset[self.tile_id_at(posn)])
        } else {
            None
        }
    }

    pub fn wrap_around(&mut self, posn: Vec2f) {
        self.position = posn
    }

    pub fn draw(&self, screen: &mut Screen) {
        let Rect {
            x: sx,
            y: sy,
            w: sw,
            h: sh,
        } = screen.bounds();
        // We'll draw from the topmost/leftmost visible tile to the bottommost/rightmost visible tile.
        // The camera combined with out position and size tell us what's visible.
        // leftmost tile: get camera.x into our frame of reference, then divide down to tile units
        // Note that it's also forced inside of 0..self.size.0
        let left = ((sx as f32 - self.position.0) / TILE_SZ as f32)
            .max(0.0)
            .min(self.dims.0 as f32) as usize;
        // rightmost tile: same deal, but with screen.x + screen.w plus a little padding to be sure we draw the rightmost tile even if it's a bit off screen.
        let right = ((sx as f32 + ((sw + TILE_SZ as u16) as f32) - self.position.0)
            / TILE_SZ as f32)
            .max(0.0)
            .min(self.dims.0 as f32) as usize;
        // ditto top and bot
        let top = ((sy as f32 - self.position.1) / TILE_SZ as f32)
            .max(0.0)
            .min(self.dims.1 as f32) as usize;
        let bot = ((sy as f32 + ((sh + TILE_SZ as u16) as f32) - self.position.1) / TILE_SZ as f32)
            .max(0.0)
            .min(self.dims.1 as f32) as usize;
        // Now draw the tiles we need to draw where we need to draw them.
        // Note that we're zipping up the row index (y) with a slice of the map grid containing the necessary rows so we can avoid making a bounds check for each tile.
        for (y, row) in (top..bot)
            .zip(self.map[(top * self.dims.0)..(bot * self.dims.0)].chunks_exact(self.dims.0))
        {
            // We are in tile coordinates at this point so we'll need to translate back to pixel units and world coordinates to draw.
            let ypx = (y * TILE_SZ) as f32 + self.position.1;
            // Here we can iterate through the column index and the relevant slice of the row in parallel
            for (x, id) in (left..right).zip(row[left..right].iter()) {
                let xpx = (x * TILE_SZ) as f32 + self.position.0;
                let frame = self.tileset.get_rect(*id);
                screen.bitblt(&self.tileset.texture, frame, Vec2f(xpx, ypx));
            }
        }
    }
}
