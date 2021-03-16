use super::ed_model::EdModel;
use crate::editor::config::Config;
use crate::editor::ed_error::EdResult;
use crate::editor::render_ast::build_code_graphics;
use crate::graphics::primitives::rect::Rect;
use crate::ui::ui_error::MissingGlyphDims;
use cgmath::Vector2;
use snafu::OptionExt;
use winit::dpi::PhysicalSize;

// create text and rectangles based on EdModel's markup_root
pub fn model_to_wgpu<'a>(
    ed_model: &'a mut EdModel,
    size: &PhysicalSize<u32>,
    txt_coords: Vector2<f32>,
    config: &Config,
) -> EdResult<(wgpu_glyph::Section<'a>, Vec<Rect>)> {
    let glyph_dim_rect = ed_model.glyph_dim_rect_opt.context(MissingGlyphDims {})?;

    build_code_graphics(
        &ed_model.markup_root,
        size,
        txt_coords,
        config,
        glyph_dim_rect,
    )
}