// MIT License
//
// Copyright (c) 2019-2023 Tobias Pfeiffer
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

#![feature(destructuring_assignment)]

#![cfg_attr(
    target_arch = "spirv",
    no_std,
    feature(register_attr),
    register_attr(spirv)
)]
// HACK(eddyb) can't easily see warnings otherwise from `spirv-builder` builds.
#![deny(warnings)]

#[cfg(not(target_arch = "spirv"))]
#[macro_use]
pub extern crate spirv_std_macros;

use {spirv_std::{*, storage_class::*, arch::*}, glam::*};

mod r#dyn;
mod sdf2d;

static CURVE_POINTS: [Vec2; 3] = [vec2(0.0, 0.0), vec2(0.5, 0.0), vec2(1.0, 1.0)];

#[spirv(vertex)]
pub fn main_vert_tri_flat(
    in_pos:        &Vec2,
    inst_model:    &Mat4,
    inst_color:    &Vec4,
    #[spirv(position)]
    mut out_pos:   &mut Vec4,
    #[spirv(flat)]
    mut out_color: &mut Vec4
) {
    *out_pos   = *inst_model * vec4(in_pos.x, in_pos.y, 0.0, 1.0);
    *out_color = *inst_color;
}

#[spirv(vertex)]
pub fn main_vert_tri_gradient(
    in_pos:        &Vec2,
    in_color:      &Vec4,
    inst_model:    &Mat4,
    inst_color:    &Vec4,
    #[spirv(position)]
    mut out_pos:   &mut Vec4,
    #[spirv(flat)]
    mut out_color: &mut Vec4
) {
    *out_pos   = *inst_model * vec4(in_pos.x, in_pos.y, 0.0, 1.0);
    *out_color = *inst_color * *in_color;
}

#[spirv(vertex)]
pub fn main_vert_tri_flat_textured(
    in_pos:        &Vec2,
    in_uv:         &Vec2,
    inst_model:    &Mat4,
    inst_color:    &Vec4,
    #[spirv(position)]
    mut out_pos:   &mut Vec4,
    #[spirv(flat)]
    mut out_color: &mut Vec4,
    mut out_uv:    &mut Vec2
) {
    *out_pos   = *inst_model * vec4(in_pos.x, in_pos.y, 0.0, 1.0);
    *out_color = *inst_color;
    *out_uv    = *in_uv;
}

#[spirv(vertex)]
pub fn main_vert_tri_gradient_textured(
    in_pos:        &Vec2,
    in_color:      &Vec4,
    in_uv:         &Vec2,
    inst_model:    &Mat4,
    inst_color:    &Vec4,
    #[spirv(position)]
    mut out_pos:   &mut Vec4,
    #[spirv(flat)]
    mut out_color: &mut Vec4,
    mut out_uv:    &mut Vec2
) {
    *out_pos   = *inst_model * vec4(in_pos.x, in_pos.y, 0.0, 1.0);
    *out_color = *inst_color * *in_color;
    *out_uv    = *in_uv;
}

#[spirv(vertex)]
pub fn main_vert_curves_flat(
    in_pos:        &Vec2,
    inst_model:    &Mat4,
    inst_color:    &Vec4,
    #[spirv(vertex_index)]
    vert_idx:      &i32,
    #[spirv(position)]
    mut out_pos:   &mut Vec4,
    #[spirv(flat)]
    mut out_color: &mut Vec4,
    mut out_uv:    &mut Vec2
) {
    *out_pos   = *inst_model * vec4(in_pos.x, in_pos.y, 0.0, 1.0);
    *out_color = *inst_color;
    *out_uv    = CURVE_POINTS[*vert_idx as usize];
}

#[spirv(vertex)]
pub fn main_vert_curves_gradient(
    in_pos:        &Vec2,
    in_color:      &Vec4,
    inst_model:    &Mat4,
    inst_color:    &Vec4,
    #[spirv(vertex_index)]
    vert_idx:      &i32,
    #[spirv(position)]
    mut out_pos:   &mut Vec4,
    #[spirv(flat)]
    mut out_color: &mut Vec4,
    mut out_uv:    &mut Vec2
) {
    *out_pos   = *inst_model * vec4(in_pos.x, in_pos.y, 0.0, 1.0);
    *out_color = *inst_color * *in_color;
    *out_uv    = CURVE_POINTS[*vert_idx as usize];
}

#[spirv(fragment)]
pub fn main_frag_tri(
    in_color:  &Vec4,
    mut out_color: &mut Vec4
) {
    *out_color = *in_color;
}

#[spirv(fragment)]
pub fn main_frag_tri_textured(
    in_color:      &Vec4,
    in_uv:         &Vec2,
    mut out_color: &mut Vec4,
    #[spirv(descriptor_set = 0, binding = 0)] texture: UniformConstant<SampledImage<Image2d>>
) {
    *out_color = { let v: Vec4 = texture.sample(*in_uv); v } * *in_color;
}

fn median(a: f32, b: f32, c: f32) -> f32 {
    a.min(b).max(a.max(b).min(c))
}

#[spirv(fragment)]
pub fn main_frag_tri_sdf(
    in_color:      &Vec4,
    in_uv:         &Vec2,
    mut out_color: &mut Vec4,
    #[spirv(descriptor_set = 0, binding = 0)] texture: UniformConstant<SampledImage<Image2d>>
) {
    //let alpha = (1.0 - texture.sample(*in_uv).r);
    //alpha = clamp((alpha - 0.9875) * 80.0, 0.0, 1.0);
    //float smoothWidth = fwidth(alpha);
    //alpha = smoothstep(0.5 - smoothWidth, 0.5 + smoothWidth, alpha);
    //*out_color = vec4(in_color.rgb, in_color.a * alpha);
    
    let s: Vec4 = texture.sample(*in_uv);
    let d = median(s.x, s.y, s.z) - 0.5;
    let w = (d / fwidth(d) + 0.5).clamp(0.0, 1.0);
    *out_color = mix(vec4(0.0, 0.0, 0.0, 0.0), in_color, w);
}

fn curve2_alpha(p: Vec2) -> f32 {
    // Gradients
    let (px, py) = (ddx(p), ddy(p));
    let f = vec2(2.0 * p.x * px.x - px.y, 2.0 * p.x * py.x - py.y); // Chain rule
    let sd = (p.x * p.x - p.y) / f.length(); // Signed distance
    return (0.5 - sd).clamp(0.0, 1.0); // Linear alpha + clamp
}

#[spirv(fragment)]
pub fn main_frag_curves(
    in_color:      &Vec4,
    in_uv:         &Vec2,
    mut out_color: &mut Vec4
) {
    *out_color = vec4(in_color.xyz(), in_color.w * curve2_alpha(*in_uv));
}