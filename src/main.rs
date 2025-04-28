use image::GrayImage;
use perlin::Perlin;

fn main() {
    let image = GrayImage::new(128, 128);

    let mut perlin = Perlin::new((128, 128), 3, image);
    perlin.generate();
}
