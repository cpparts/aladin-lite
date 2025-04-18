pub type Url = String;

use super::request::RequestType;
pub trait Query: Sized {
    type Request: From<Self> + Into<RequestType>;

    fn id(&self) -> &QueryId;
}

pub type QueryId = String;

use al_core::image::format::ImageFormatType;

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Tile {
    pub cell: HEALPixCell,
    pub format: ImageFormatType,
    // The root url of the HiPS
    pub hips_cdid: CreatorDid,
    // The total url of the query
    pub url: Url,
    pub id: QueryId,
    pub channel: Option<u32>,
}

use crate::healpix::cell::HEALPixCell;
use crate::renderable::hips::config::HiPSConfig;
use crate::renderable::CreatorDid;
impl Tile {
    pub fn new(cell: &HEALPixCell, channel: Option<u32>, cfg: &HiPSConfig) -> Self {
        let hips_cdid = cfg.get_creator_did();
        let hips_url = cfg.get_root_url();
        let format = cfg.get_format();

        let ext = format.get_ext_file();

        let HEALPixCell(depth, idx) = *cell;

        let dir_idx = (idx / 10000) * 10000;

        let mut url = format!("{}/Norder{}/Dir{}/Npix{}", hips_url, depth, dir_idx, idx);

        // handle cube case
        if let Some(channel) = channel {
            if channel > 0 {
                url.push_str(&format!("_{:?}", channel));
            }
        }

        // add the tile format
        url.push_str(&format!(".{}", ext));

        let id = format!(
            "{}{}{}{}{}",
            hips_cdid,
            depth,
            idx,
            channel.unwrap_or(0),
            ext
        );

        Tile {
            hips_cdid: hips_cdid.to_string(),
            url,
            cell: *cell,
            format,
            id,
            channel,
        }
    }
}

use super::request::tile::TileRequest;
impl Query for Tile {
    type Request = TileRequest;

    fn id(&self) -> &QueryId {
        &self.id
    }
}

/* ---------------------------------- */
pub struct Allsky {
    pub format: ImageFormatType,
    pub tile_size: i32,
    pub texture_size: i32,
    pub channel: Option<u32>,
    // The root url of the HiPS
    pub hips_cdid: CreatorDid,
    // The total url of the query
    pub url: Url,
    pub id: QueryId,
}

impl Allsky {
    pub fn new(cfg: &HiPSConfig, channel: Option<u32>) -> Self {
        let hips_cdid = cfg.get_creator_did().to_string();
        let tile_size = cfg.get_tile_size();
        let texture_size = cfg.get_texture_size();
        let format = cfg.get_format();
        let ext = format.get_ext_file();

        let mut url = format!("{}/Norder3/Allsky", cfg.get_root_url());

        // handle cube case
        if let Some(channel) = channel {
            if channel > 0 {
                url.push_str(&format!("_{:?}", channel));
            }
        }

        // add the tile format
        url.push_str(&format!(".{}", ext));

        let id = format!(
            "{}Allsky{}{}",
            cfg.get_creator_did(),
            ext,
            channel.unwrap_or(0)
        );

        Allsky {
            tile_size,
            texture_size,
            hips_cdid,
            url,
            format,
            id,
            channel,
        }
    }
}

use super::request::allsky::AllskyRequest;
impl Query for Allsky {
    type Request = AllskyRequest;

    fn id(&self) -> &QueryId {
        &self.id
    }
}

/* ---------------------------------- */
pub struct PixelMetadata {
    pub format: ImageFormatType,
    // The root url of the HiPS
    pub hips_cdid: CreatorDid,
    // The total url of the query
    pub url: Url,
    pub id: QueryId,
}

impl PixelMetadata {
    pub fn new(cfg: &HiPSConfig) -> Self {
        let hips_cdid = cfg.get_creator_did().to_string();
        let format = cfg.get_format();
        let ext = format.get_ext_file();

        let url = format!("{}/Norder3/Allsky.{}", cfg.get_root_url(), ext);

        let id = format!("{}Allsky{}", hips_cdid, ext);
        PixelMetadata {
            hips_cdid,
            url,
            format,
            id,
        }
    }
}

use super::request::blank::PixelMetadataRequest;
impl Query for PixelMetadata {
    type Request = PixelMetadataRequest;

    fn id(&self) -> &QueryId {
        &self.id
    }
}
use al_api::moc::MOCOptions;
/* ---------------------------------- */
pub struct Moc {
    // The total url of the query
    pub url: Url,
    pub params: MOCOptions,
    pub hips_cdid: CreatorDid,
}
impl Moc {
    pub fn new(url: String, hips_cdid: CreatorDid, params: MOCOptions) -> Self {
        Moc {
            url,
            params,
            hips_cdid,
        }
    }
}

use super::request::moc::MOCRequest;
impl Query for Moc {
    type Request = MOCRequest;

    fn id(&self) -> &QueryId {
        &self.url
    }
}
