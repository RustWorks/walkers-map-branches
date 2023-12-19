#![doc = include_str!("../README.md")]
#![deny(clippy::unwrap_used, rustdoc::broken_intra_doc_links)]

mod download;
pub mod extras;
mod io;
mod map;
mod mercator;
pub mod providers;
mod tiles;
mod zoom;

pub use map::{Map, MapMemory, Plugin, Projector};
pub use mercator::{screen_to_position, Position, TileId};
pub use tiles::{Tiles, TilesManager, Texture};
pub use zoom::InvalidZoom;
