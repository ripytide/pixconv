use rgb::Rgb;

use crate::{
    color::rgb::RgbColorSpace,
    traits::{HasColorSpace, PixelConvert},
    Image,
};

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
        image: Image<Rgb<u8>>,
        source_colorspace: <Rgb<u8> as HasColorSpace>::ColorSpace,
        destination_colorspace: RgbColorSpace,
    ) -> Image<Self> {
        todo!()
    }
    fn pixel_convert_image_with(
        input_image: &Image<Rgb<u8>>,
        output_image: &mut Image<Self>,
        source_colorspace: RgbColorSpace,
        destination_colorspace: RgbColorSpace,
    ) {
        todo!()
    }
}
