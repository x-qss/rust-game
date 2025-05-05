use std::f64::consts::PI;
use web_sys::{ CanvasRenderingContext2d, HtmlCanvasElement, MessageEvent, WebSocket, window };

use crate::structs::player::Player;

pub fn render(
    ctx: &CanvasRenderingContext2d,
    dt: f64,
    me: &mut Player,
    canvas: &HtmlCanvasElement
) {
    let (x_offset, y_offset) = (
        me.x - (canvas.width() as f64) / 2.0,
        me.y - (canvas.height() as f64) / 2.0,
    );

    // render outside
    ctx.save();
    ctx.set_fill_style_str("#000");
    ctx.set_global_alpha(0.25);
    ctx.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    ctx.fill();
    ctx.restore();

    // render snow biome
    ctx.set_fill_style_str("#fff");
    ctx.fill_rect(0.0 - x_offset, 0.0 - y_offset, 14400.0, 2400.0);
    ctx.fill();

    // render grasslands
    ctx.set_fill_style_str("#8ecc51");
    ctx.fill_rect(0.0 - x_offset, 2400.0 - y_offset, 14400.0, 14400.0 - 2400.0);
    ctx.fill();

    // render desert
    ctx.set_fill_style_str("#ff0000");
    ctx.fill_rect(0.0 - x_offset, 14400.0 - 2400.0 - y_offset, 14400.0, 2400.0);
    ctx.fill();

    // grid lines
    ctx.save();
    ctx.set_fill_style_str("white");
    ctx.set_line_width(4.0);
    ctx.set_global_alpha(0.06);
    ctx.begin_path();
    for i in (-(me.x as i64)..canvas.width() as i64).step_by(
        (canvas.height() / 12).try_into().unwrap()
    ) {
        if i > 0 {
            ctx.move_to(i as f64, 0.0);
            ctx.line_to(i as f64, canvas.height() as f64);
        }
    }

    for i in (-(me.y as i64)..canvas.height() as i64).step_by(
        (canvas.height() / 12).try_into().unwrap()
    ) {
        if i > 0 {
            ctx.move_to(0.0, i as f64);
            ctx.line_to(canvas.width() as f64, i as f64);
        }
    }

    ctx.stroke();
    ctx.restore();

    // overlay screen to darken colors
    ctx.save();
    ctx.set_fill_style_str("#000046");
    ctx.set_global_alpha(0.15);
    ctx.fill_rect(0.0 - x_offset, 0.0 - y_offset, canvas.width() as f64, canvas.height() as f64);
    ctx.fill();
    ctx.restore();

    // todo: implement exponential decay for consistent lerping across framerates
    // apply lerp: curr.xy + (end.xy - curr.xy) * time
    me.vx = me.lx + (me.x - me.lx) * dt;
    me.vy = me.ly + (me.y - me.ly) * dt;

    ctx.begin_path();
    ctx.set_line_width(5.5);
    ctx.set_stroke_style_str("#525252");
    ctx.set_fill_style_str("red");
    let _ = ctx.arc(me.vx - x_offset, me.vy - y_offset, 35.0, 0.0, PI * 2.0);
    ctx.stroke();
    ctx.fill();
}
