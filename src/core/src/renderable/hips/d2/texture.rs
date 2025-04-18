use crate::{healpix::cell::HEALPixCell, time::Time};

use std::collections::HashSet;

pub struct HpxTexture2D {
    texture_cell: HEALPixCell,
    // Precomputed uniq number
    uniq: i32,
    // The cells located in the Texture
    tiles: HashSet<HEALPixCell>,
    // Position of the texture in the buffer
    idx: i32,
    // The time the texture has been received
    // If the texture contains multiple tiles, then the receiving time
    // is set when all the tiles have been copied to the buffer
    start_time: Option<Time>,
    // The time request of the texture is the time request
    // of the first tile being inserted in it
    // It is then only given in the constructor of Texture
    // This is approximate, it should correspond to the minimum
    // of the time requests of the cells currenlty contained in the
    // texture. But this is too expensive because at each tile inserted
    // in the buffer, one should reevalute the priority of the texture
    // in the buffer's binary heap.
    time_request: Time,

    // Full flag telling the texture has been filled
    full: bool,

    // Num tiles written for the gpu
    num_tiles_written: usize,
    // Flag telling whether the texture is available
    // for drawing
    //missing: bool,
}

use crate::renderable::hips::config::HiPSConfig;

use crate::renderable::hips::HpxTile;

impl HpxTexture2D {
    pub fn new(cell: &HEALPixCell, idx: i32, time_request: Time) -> Self {
        let tiles = HashSet::new();

        let start_time = None;
        let full = false;
        let texture_cell = *cell;
        let uniq = texture_cell.uniq();
        //let missing = true;
        let num_tiles_written = 0;

        Self {
            texture_cell,
            uniq,
            time_request,
            tiles,
            idx,
            start_time,
            full,
            num_tiles_written,
        }
    }

    pub fn is_full(&self) -> bool {
        self.full
    }

    pub fn idx(&self) -> i32 {
        self.idx
    }

    // Setter
    pub fn replace(&mut self, texture_cell: &HEALPixCell, time_request: Time) {
        // Cancel the tasks copying the tiles contained in the texture
        // which have not yet been completed.
        //self.clear_tasks_in_progress(config, exec);

        self.texture_cell = *texture_cell;
        self.uniq = texture_cell.uniq();
        self.full = false;
        self.start_time = None;
        self.time_request = time_request;
        self.tiles.clear();
        //self.missing = true;
        self.num_tiles_written = 0;
    }

    // Cell must be contained in the texture
    pub fn contains_tile(&self, tile_cell: &HEALPixCell) -> bool {
        self.is_full() || self.tiles.contains(tile_cell)
    }

    // Panic if cell is not contained in the texture
    // Do nothing if the texture is full
    // Return true if the tile is newly added
    pub fn append(&mut self, cell: &HEALPixCell, cfg: &HiPSConfig) {
        let texture_cell = cell.get_texture_cell(cfg.delta_depth());
        debug_assert!(texture_cell == self.texture_cell);
        debug_assert!(!self.full);

        //self.missing &= missing;
        //self.start_time = Some(Time::now());
        //self.full = true;
        let num_tiles_per_texture = cfg.num_tiles_per_texture();
        let c = *cell;

        if c == texture_cell {
            self.num_tiles_written = num_tiles_per_texture;
            self.full = true;

            self.start_time = Some(Time::now());
        } else {
            // Sub-tile appending. This code is called for tile size is < 512
            // Cell has the good ancestor for this texture
            let new_tile = self.tiles.insert(c);
            // Ensures the tile was not already present in the buffer
            // This is the case because already contained cells do not
            // lead to new requests
            debug_assert!(new_tile);
            self.num_tiles_written += 1;

            if self.num_tiles_written == num_tiles_per_texture {
                // The texture is full and available
                self.full = true;
                self.start_time = Some(Time::now());
            }
        }
    }
}

impl HpxTile for HpxTexture2D {
    // Getter
    // Returns the current time if the texture is not full
    fn start_time(&self) -> Time {
        if self.is_full() {
            self.start_time.unwrap_abort()
        } else {
            Time::now()
        }
    }

    fn time_request(&self) -> Time {
        self.time_request
    }

    fn cell(&self) -> &HEALPixCell {
        &self.texture_cell
    }
}

use std::cmp::Ordering;
impl PartialOrd for HpxTexture2D {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.uniq.partial_cmp(&other.uniq)
    }
}
use crate::Abort;
impl Ord for HpxTexture2D {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_abort()
    }
}

impl PartialEq for HpxTexture2D {
    fn eq(&self, other: &Self) -> bool {
        self.uniq == other.uniq
    }
}
impl Eq for HpxTexture2D {}

pub struct HpxTexture2DUniforms<'a> {
    texture: &'a HpxTexture2D,
    name: String,
}

impl<'a> HpxTexture2DUniforms<'a> {
    pub fn new(texture: &'a HpxTexture2D, idx_texture: i32) -> Self {
        let name = format!("textures_tiles[{}].", idx_texture);
        HpxTexture2DUniforms { texture, name }
    }
}

use al_core::shader::{SendUniforms, ShaderBound};
impl<'a> SendUniforms for HpxTexture2DUniforms<'a> {
    // Info: These uniforms are used for raytracing drawing mode only
    fn attach_uniforms<'b>(&self, shader: &'b ShaderBound<'b>) -> &'b ShaderBound<'b> {
        shader
            .attach_uniform(&format!("{}{}", self.name, "uniq"), &self.texture.uniq)
            .attach_uniform(
                &format!("{}{}", self.name, "texture_idx"),
                &self.texture.idx,
            )
            .attach_uniform(
                &format!("{}{}", self.name, "empty"),
                // This is useful for FITS tiles only because:
                // - for JPEG, missing tiles are inserted in the buffer and black is drawn
                // - for PNG, tiles are not inserted but default color chosen is fully transparent (might be vec4(0.0, 0.0, 0.0, 0.0))
                // 
                // Therefore for FITS files we must indicate GPU which base tiles are missing so that we draw fully transparent pixels
                &((!self.texture.full as u8) as f32),
            )
            .attach_uniform(
                &format!("{}{}", self.name, "start_time"),
                &self.texture.start_time(),
            );

        shader
    }
}
