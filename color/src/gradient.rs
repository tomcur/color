// Copyright 2024 the Color Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::{Colorspace, ColorspaceTag, CssColor, HueDirection, Interpolator, Oklab, PremulColor};

#[expect(missing_debug_implementations, reason = "it's an iterator")]
pub struct GradientIter<CS: Colorspace> {
    interpolator: Interpolator,
    // This is in deltaEOK units
    tolerance: f32,
    // The adaptive subdivision logic is lifted from the stroke expansion paper.
    t0: u32,
    dt: f32,
    target0: PremulColor<CS>,
    target1: PremulColor<CS>,
    end_color: PremulColor<CS>,
}

pub fn gradient<CS: Colorspace>(
    mut color0: CssColor,
    mut color1: CssColor,
    interp_cs: ColorspaceTag,
    direction: HueDirection,
    tolerance: f32,
) -> GradientIter<CS> {
    let interpolator = color0.interpolate(color1, interp_cs, direction);
    if color0.missing.any() {
        color0 = interpolator.eval(0.0);
    }
    let target0 = color0.to_alpha_color().premultiply();
    if color1.missing.any() {
        color1 = interpolator.eval(1.0);
    }
    let target1 = color1.to_alpha_color().premultiply();
    let end_color = target1;
    GradientIter {
        interpolator,
        tolerance,
        t0: 0,
        dt: 0.0,
        target0,
        target1,
        end_color,
    }
}

impl<CS: Colorspace> Iterator for GradientIter<CS> {
    type Item = (f32, PremulColor<CS>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.dt == 0.0 {
            self.dt = 1.0;
            return Some((0.0, self.target0));
        }
        let t0 = self.t0 as f32 * self.dt;
        if t0 == 1.0 {
            return None;
        }
        loop {
            // compute midpoint color
            let midpoint = self.interpolator.eval(t0 + 0.5 * self.dt);
            let midpoint_oklab: PremulColor<Oklab> = midpoint.to_alpha_color().premultiply();
            let approx = self.target0.lerp_rect(self.target1, 0.5);
            let error = midpoint_oklab.difference(approx.convert());
            if error <= self.tolerance {
                let t1 = t0 + self.dt;
                self.t0 += 1;
                let shift = self.t0.trailing_zeros();
                self.t0 >>= shift;
                self.dt *= (1 << shift) as f32;
                self.target0 = self.target1;
                let new_t1 = t1 + self.dt;
                if new_t1 < 1.0 {
                    self.target1 = self
                        .interpolator
                        .eval(new_t1)
                        .to_alpha_color()
                        .premultiply();
                } else {
                    self.target1 = self.end_color;
                }
                return Some((t1, self.target0));
            }
            self.t0 *= 2;
            self.dt *= 0.5;
            self.target1 = midpoint.to_alpha_color().premultiply();
        }
    }
}
