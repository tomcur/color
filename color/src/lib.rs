// Copyright 2024 the Color Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// LINEBENDER LINT SET - v1
// See https://linebender.org/wiki/canonical-lints/
// These lints aren't included in Cargo.toml because they
// shouldn't apply to examples and tests
#![warn(unused_crate_dependencies)]
#![warn(clippy::print_stdout, clippy::print_stderr)]
// TODO: parts of the crate are not done, with some missing docstring,
// and some enum variants not yet implemented. Finish those and remove
// these allow attributes.
#![allow(missing_docs, reason = "need to write more docs")]
#![allow(clippy::todo, reason = "need to fix todos")]

//! # Color
//!
//! TODO: need to write a treatise on the nature of color and how to model
//! a reasonable fragment of it in the Rust type system.

mod bitset;
mod color;
mod colorspace;
mod css;
mod gradient;
// Note: this will be feature-gated, but not bothering for now
mod parse;
mod serialize;
mod tagged;

#[cfg(all(not(feature = "std"), not(test)))]
mod floatfuncs;

pub use bitset::Bitset;
pub use color::{AlphaColor, HueDirection, OpaqueColor, PremulColor};
pub use colorspace::{
    Colorspace, ColorspaceLayout, DisplayP3, LinearSrgb, Oklab, Oklch, Srgb, XyzD65,
};
pub use css::{CssColor, Interpolator};
pub use gradient::{gradient, GradientIter};
pub use parse::{parse_color, Error};
pub use tagged::{ColorspaceTag, TaggedColor};

const fn u8_to_f32(x: u32) -> f32 {
    x as f32 * (1.0 / 255.0)
}

fn matmul(m: &[[f32; 3]; 3], x: [f32; 3]) -> [f32; 3] {
    [
        m[0][0] * x[0] + m[0][1] * x[1] + m[0][2] * x[2],
        m[1][0] * x[0] + m[1][1] * x[1] + m[1][2] * x[2],
        m[2][0] * x[0] + m[2][1] * x[1] + m[2][2] * x[2],
    ]
}

impl AlphaColor<Srgb> {
    pub const fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        let components = [
            u8_to_f32(r as u32),
            u8_to_f32(g as u32),
            u8_to_f32(b as u32),
            u8_to_f32(a as u32),
        ];
        Self::new(components)
    }
}

// Keep clippy from complaining about unused libm in nostd test case.
#[cfg(feature = "libm")]
#[expect(unused, reason = "keep clippy happy")]
fn ensure_libm_dependency_used() -> f32 {
    libm::sqrtf(4_f32)
}
