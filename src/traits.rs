use pixmeta::Pixels;

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
        image: Pixels<P>,
        source_colorspace: P::ColorSpace,
        destination_colorspace: Self::ColorSpace,
    ) -> Pixels<Self>;
}
