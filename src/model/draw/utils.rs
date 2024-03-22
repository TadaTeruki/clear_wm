use cairo::{LinearGradient, RadialGradient};

use crate::model::client::geometry::{self, Geometry};

enum Alignment {
    Top,
    Bottom,
    Left,
    Right,
}

fn drop_shadow_side(
    context: &cairo::Context,
    color_outer: (f64, f64, f64, f64),
    color_inner: (f64, f64, f64, f64),
    alignment: Alignment,
    outer_geom: &Geometry,
    inner_geom: &Geometry,
    border_width: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let geom = match alignment {
        Alignment::Top => geometry::Geometry {
            x: inner_geom.x,
            y: outer_geom.y,
            width: inner_geom.width,
            height: border_width,
        },
        Alignment::Bottom => geometry::Geometry {
            x: inner_geom.x,
            y: inner_geom.y + inner_geom.height as i32,
            width: inner_geom.width,
            height: border_width,
        },
        Alignment::Left => geometry::Geometry {
            x: outer_geom.x,
            y: inner_geom.y,
            width: border_width,
            height: inner_geom.height,
        },
        Alignment::Right => geometry::Geometry {
            x: inner_geom.x + inner_geom.width as i32,
            y: inner_geom.y,
            width: border_width,
            height: inner_geom.height,
        },
    };

    let pat = match alignment {
        Alignment::Top | Alignment::Bottom => LinearGradient::new(
            geom.x as f64,
            geom.y as f64,
            geom.x as f64,
            geom.y as f64 + geom.height as f64,
        ),
        Alignment::Left | Alignment::Right => LinearGradient::new(
            geom.x as f64,
            geom.y as f64,
            geom.x as f64 + geom.width as f64,
            geom.y as f64,
        ),
    };

    match alignment {
        Alignment::Top | Alignment::Left => {
            pat.add_color_stop_rgba(
                0.0,
                color_outer.0,
                color_outer.1,
                color_outer.2,
                color_outer.3,
            );
            pat.add_color_stop_rgba(
                1.0,
                color_inner.0,
                color_inner.1,
                color_inner.2,
                color_inner.3,
            );
        }
        Alignment::Bottom | Alignment::Right => {
            pat.add_color_stop_rgba(
                0.0,
                color_inner.0,
                color_inner.1,
                color_inner.2,
                color_inner.3,
            );
            pat.add_color_stop_rgba(
                1.0,
                color_outer.0,
                color_outer.1,
                color_outer.2,
                color_outer.3,
            );
        }
    }

    context.set_source(&pat)?;

    context.rectangle(
        geom.x as f64,
        geom.y as f64,
        geom.width as f64,
        geom.height as f64,
    );

    context.fill()?;

    Ok(())
}

fn drop_shadow_corner(
    context: &cairo::Context,
    color_outer: (f64, f64, f64, f64),
    color_inner: (f64, f64, f64, f64),
    center_x: f64,
    center_y: f64,
    radius: f64,
    top: bool,
    left: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let (start_angle, end_angle) = match (top, left) {
        (true, true) => (1.0 * std::f64::consts::PI, 1.5 * std::f64::consts::PI),
        (true, false) => (1.5 * std::f64::consts::PI, 2.0 * std::f64::consts::PI),
        (false, true) => (0.5 * std::f64::consts::PI, 1.0 * std::f64::consts::PI),
        (false, false) => (0.0, 0.5 * std::f64::consts::PI),
    };

    let pat = RadialGradient::new(center_x, center_y, 0.0, center_x, center_y, radius);

    pat.add_color_stop_rgba(
        0.0,
        color_inner.0,
        color_inner.1,
        color_inner.2,
        color_inner.3,
    );
    pat.add_color_stop_rgba(
        1.0,
        color_outer.0,
        color_outer.1,
        color_outer.2,
        color_outer.3,
    );

    context.set_source(&pat)?;
    context.arc(center_x, center_y, radius, start_angle, end_angle);
    context.line_to(center_x, center_y);

    let last_point = match (top, left) {
        (true, true) => (center_x - radius, center_y),
        (true, false) => (center_x, center_y - radius),
        (false, true) => (center_x, center_y + radius),
        (false, false) => (center_x + radius, center_y),
    };

    context.line_to(last_point.0, last_point.1);

    context.fill()?;
    Ok(())
}

pub fn drop_shadow(
    context: &cairo::Context,
    outer_geom: &Geometry,
    inner_geom: &Geometry,
    color_outer: (f64, f64, f64, f64),
    color_inner: (f64, f64, f64, f64),
) -> Result<(), Box<dyn std::error::Error>> {
    let border_width = (inner_geom.x - outer_geom.x) as u32;

    // top left
    drop_shadow_corner(
        context,
        color_outer,
        color_inner,
        inner_geom.x as f64,
        inner_geom.y as f64,
        border_width as f64,
        true,
        true,
    )?;

    // top right
    drop_shadow_corner(
        context,
        color_outer,
        color_inner,
        inner_geom.x as f64 + inner_geom.width as f64,
        inner_geom.y as f64,
        border_width as f64,
        true,
        false,
    )?;

    // bottom left
    drop_shadow_corner(
        context,
        color_outer,
        color_inner,
        inner_geom.x as f64,
        inner_geom.y as f64 + inner_geom.height as f64,
        border_width as f64,
        false,
        true,
    )?;

    // bottom right
    drop_shadow_corner(
        context,
        color_outer,
        color_inner,
        inner_geom.x as f64 + inner_geom.width as f64,
        inner_geom.y as f64 + inner_geom.height as f64,
        border_width as f64,
        false,
        false,
    )?;

    // top
    drop_shadow_side(
        context,
        color_outer,
        color_inner,
        Alignment::Top,
        outer_geom,
        inner_geom,
        border_width,
    )?;

    // bottom
    drop_shadow_side(
        context,
        color_outer,
        color_inner,
        Alignment::Bottom,
        outer_geom,
        inner_geom,
        border_width,
    )?;

    // left
    drop_shadow_side(
        context,
        color_outer,
        color_inner,
        Alignment::Left,
        outer_geom,
        inner_geom,
        border_width,
    )?;

    // right
    drop_shadow_side(
        context,
        color_outer,
        color_inner,
        Alignment::Right,
        outer_geom,
        inner_geom,
        border_width,
    )?;

    Ok(())
}
