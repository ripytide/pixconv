use super::{Luminance, Primaries, Transfer, Whitepoint};

/// An rgb-ish, additive model based on the CIE 1931 XYZ observers.
///
/// The _linear_ representation is the screen space linear RGB, which depends on primaries,
/// whitepoint and reference luminance. It is derived from the encoded form through the
/// transfer function.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RgbColorSpace {
    primary: Primaries,
    transfer: Transfer,
    whitepoint: Whitepoint,
    luminance: Luminance,
}
impl RgbColorSpace {
    pub const SRGB: Self = Self {
        luminance: Luminance::Sdr,
        primary: Primaries::Bt709,
        transfer: Transfer::Srgb,
        whitepoint: Whitepoint::D65,
    };

    pub const BT709_RGB: Self = Self {
        luminance: Luminance::Sdr,
        primary: Primaries::Bt709,
        transfer: Transfer::Bt709,
        whitepoint: Whitepoint::D65,
    };
}
