pub mod component_physics;
pub mod component_position;
pub mod component_render;
pub mod component_render_line_buffer;
pub mod component_render_text;
pub mod component_render_triangle_buffer;
pub mod entity;
pub mod system_physics;
pub mod system_render;
pub mod world;

use entity::*;
pub use world::*;

pub use component_physics::*;
pub use component_position::*;
use component_render::*;
use component_render_line_buffer::*;
use component_render_text::*;
use component_render_triangle_buffer::*;

pub use system_physics::*;
pub use system_render::*;

use super::cache::CacheManager;
use super::fonts;
use super::support::Gl;

pub const COMPONENT_ACTIVE: u32 = 1 << 0;
pub const COMPONENT_POSITION: u32 = 1 << 1;
pub const COMPONENT_RENDER: u32 = 1 << 2;
pub const COMPONENT_VISIBLE: u32 = 1 << 3;
pub const COMPONENT_PHYSICS: u32 = 1 << 4;
