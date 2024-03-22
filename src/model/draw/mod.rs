mod utils;

use crate::config::FrameConfig;

use super::client::{
    geometry::{ClientGeometry, Geometry},
    hints::ClientHints,
};

pub struct FrameDrawContext {
    pub context: cairo::Context,
}

impl FrameDrawContext {
    pub fn new(context: cairo::Context) -> Self {
        Self { context }
    }

    pub fn draw(
        &self,
        geometry: &ClientGeometry,
        frame_config: &FrameConfig,
        hints: &ClientHints,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.context.set_operator(cairo::Operator::Source);
        let outer_frame_draw_area = geometry.parse_as_outer_frame_draw_area();
        let inner_frame_draw_area = geometry.parse_as_inner_frame_draw_area();

        let drop_shadow_shrink_ratio = 0.7;
        let drop_shadow_shrink_amount = frame_config.border_width as f64 * drop_shadow_shrink_ratio;
        let shrink_outer_geom = Geometry {
            x: outer_frame_draw_area.x + drop_shadow_shrink_amount as i32,
            y: outer_frame_draw_area.y + drop_shadow_shrink_amount as i32,
            width: outer_frame_draw_area.width - drop_shadow_shrink_amount as u32,
            height: outer_frame_draw_area.height - drop_shadow_shrink_amount as u32,
        };

        utils::drop_shadow(
            &self.context,
            &shrink_outer_geom,
            &inner_frame_draw_area,
            (0.0, 0.0, 0.0, 0.0),
            (0.0, 0.0, 0.0, 0.3),
        )?;

        // draw inner frame
        self.context.set_source_rgba(0.5, 0.4, 0.3, 1.0);
        self.context.rectangle(
            inner_frame_draw_area.x as f64,
            inner_frame_draw_area.y as f64,
            inner_frame_draw_area.width as f64,
            inner_frame_draw_area.height as f64,
        );

        self.context.fill()?;

        let app_draw_area = geometry.parse_as_app_draw_area();

        // draw app
        self.context.set_source_rgba(1.0, 1.0, 1.0, 0.5);
        self.context.rectangle(
            app_draw_area.x as f64,
            app_draw_area.y as f64,
            app_draw_area.width as f64,
            app_draw_area.height as f64,
        );

        self.context.fill()?;

        let title_margin = frame_config.titlebar_height as f64 * 0.2;
        let title_font_size = frame_config.titlebar_height as f64 * 0.6;

        // draw title in hints
        self.context.set_source_rgba(0.95, 0.95, 0.95, 1.0);
        self.context.move_to(
            inner_frame_draw_area.x as f64 + title_margin,
            inner_frame_draw_area.y as f64 + frame_config.titlebar_height as f64 - title_margin,
        );
        // font size
        self.context.set_font_size(title_font_size);
        // font family
        self.context
            .select_font_face("Arial", cairo::FontSlant::Normal, cairo::FontWeight::Bold);
        self.context.show_text(&hints.title)?;

        Ok(())
    }
}
