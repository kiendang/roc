use super::ed_model::EdModel;
use crate::editor::code_lines::CodeLines;
use crate::editor::config::Config;
use crate::editor::ed_error::EdResult;
use crate::editor::render_ast::build_code_graphics;
use crate::editor::slow_pool::SlowPool;
use crate::graphics::primitives::rect::Rect;
use crate::ui::text::caret_w_select::make_caret_rect;
use crate::ui::text::caret_w_select::CaretWSelect;
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
    markup_node_pool: &'a SlowPool,
) -> EdResult<(wgpu_glyph::Section<'a>, Vec<Rect>)> {
    let glyph_dim_rect = ed_model.glyph_dim_rect_opt.context(MissingGlyphDims {})?;

    let (section, mut rects) = build_code_graphics(
        markup_node_pool.get(ed_model.markup_root_id),
        size,
        txt_coords,
        config,
        glyph_dim_rect,
        markup_node_pool,
    )?;

    let mut all_code_string = String::new();

    for txt in section.text.iter() {
        all_code_string.push_str(txt.text);
    }

    ed_model.code_lines = CodeLines::from_str(&all_code_string);

    let caret_w_sel_vec = ed_model
        .caret_w_select_vec
        .iter()
        .map(|(caret_w_sel, _)| *caret_w_sel)
        .collect();

    let mut sel_rects =
        build_selection_graphics(caret_w_sel_vec, txt_coords, config, glyph_dim_rect)?;

    rects.append(&mut sel_rects);

    Ok((section, rects))
}

pub fn build_selection_graphics(
    caret_w_select_vec: Vec<CaretWSelect>,
    txt_coords: Vector2<f32>,
    config: &Config,
    glyph_dim_rect: Rect,
) -> EdResult<Vec<Rect>> {
    let mut rects = Vec::new();
    let char_width = glyph_dim_rect.width;
    let char_height = glyph_dim_rect.height;

    for caret_w_sel in caret_w_select_vec {
        let caret_row = caret_w_sel.caret_pos.line as f32;
        let caret_col = caret_w_sel.caret_pos.column as f32;

        let top_left_x = txt_coords.x + caret_col * char_width;

        let top_left_y = txt_coords.y + caret_row * char_height;

        rects.push(make_caret_rect(
            top_left_x,
            top_left_y,
            &glyph_dim_rect,
            &config.ed_theme.ui_theme,
        ))
    }

    Ok(rects)
}
