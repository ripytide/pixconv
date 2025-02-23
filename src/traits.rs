use crate::Image;

pub trait HasColorSpace {
    type ColorSpace;
}

pub trait PixelConvert<P>: HasColorSpace + Sized
where
    P: HasColorSpace,
{
    #[must_use]
    fn pixel_convert(
        pixel: P,
        source_colorspace: P::ColorSpace,
        destination_colorspace: Self::ColorSpace,
    ) -> Self;

    #[must_use]
    fn pixel_convert_image(
        image: Image<P>,
        source_colorspace: P::ColorSpace,
        destination_colorspace: Self::ColorSpace,
    ) -> Image<Self>;

    fn pixel_convert_image_with(
        input_image: &Image<P>,
        output_image: &mut Image<Self>,
        source_colorspace: P::ColorSpace,
        destination_colorspace: Self::ColorSpace,
    );
}
