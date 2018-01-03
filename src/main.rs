#![recursion_limit="2048"]

#[macro_use]
extern crate stdweb;

use stdweb::web;
use std::f32::consts::PI;


fn draw_circle(ctx: &stdweb::Value, color: &str, pos: (f32, f32),
               radius: f32, angle: (f32, f32), anticlockwise: bool) {
    js!(
        @{ctx}.beginPath();
        @{ctx}.arc(@{pos.0}, @{pos.1}, @{radius},
                   @{angle.0}, @{angle.1}, @{anticlockwise});
        @{ctx}.fillStyle = @{color};
        @{ctx}.fill();
    );
}

fn main() {
    stdweb::initialize();

    let canvas = web::document().get_element_by_id("viewport").unwrap();
    let ctx: stdweb::Value = js!( return @{canvas}.getContext("2d"); );

    draw_circle(&ctx, "#B7282E", (40.0, 40.0), 40.0, (0.0, 2.0*PI), true);
    draw_circle(&ctx, "#0086CC", (120.0, 40.0), 40.0, (PI/2.0, 3.0*PI/2.0), false);
    draw_circle(&ctx, "#B168A8", (40.0, 120.0), 40.0, (0.0, PI), true);
    draw_circle(&ctx, "#F3BF88", (120.0, 120.0), 40.0, (-PI, -PI/2.0), false);
}
