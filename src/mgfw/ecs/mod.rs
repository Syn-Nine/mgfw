pub mod component_angle;
pub mod component_color;
pub mod component_easing;
pub mod component_physics;
pub mod component_position;
pub mod component_render;
pub mod component_render_billboard;
pub mod component_render_line_buffer;
pub mod component_render_text;
pub mod component_render_triangle_buffer;
pub mod component_scale;
pub mod entity;
pub mod system_easing;
pub mod system_physics;
pub mod system_render;
pub mod world;

use entity::*;
pub use world::*;

pub use component_angle::*;
pub use component_color::*;
use component_easing::*;
pub use component_physics::*;
pub use component_position::*;
use component_render::*;
use component_render_billboard::*;
use component_render_line_buffer::*;
use component_render_text::*;
use component_render_triangle_buffer::*;
pub use component_scale::*;

pub use system_easing::*;
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
pub const COMPONENT_ANGLE: u32 = 1 << 5;
pub const COMPONENT_SCALE: u32 = 1 << 6;
pub const COMPONENT_COLOR: u32 = 1 << 7;
