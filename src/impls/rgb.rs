use pixmeta::{Pixels, RgbColorSpace};
use rgb::Rgb;

use crate::traits::{HasColorSpace, PixelConvert};

impl<T> HasColorSpace for Rgb<T> {
    type ColorSpace = RgbColorSpace;
}

impl PixelConvert<Rgb<u8>> for Rgb<u16> {
    fn pixel_convert(
        pixel: Rgb<u8>,
        source_colorspace: RgbColorSpace,
        destination_colorspace: RgbColorSpace,
    ) -> Self {
        todo!()
    }

    fn pixel_convert_image(
        image: Pixels<Rgb<u8>>,
        source_colorspace: <Rgb<u8> as HasColorSpace>::ColorSpace,
        destination_colorspace: RgbColorSpace,
    ) -> Pixels<Self> {
        todo!()
    }
}
