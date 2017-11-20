
extern crate image;
use image::{ DynamicImage, GenericImage };
use image::png::PNGEncoder;
use rand::StdRng;

mod images;
use image_generators::images::{circle_set, square_set, shapes_generate};


#[inline]
/// Returns a png image
fn image_to_vec(image: &DynamicImage) -> Vec<u8> {
    let mut x: Vec<u8> = vec![];
    PNGEncoder::new(&mut x)
        .encode(
            image.raw_pixels().as_slice(),
            image.width(),
            image.height(),
            image.color(),
        )
        .ok();
    x
}

pub fn imageset_delegator(itemset: &str, mut rng: &mut StdRng) -> Result<Vec<u8>, &'static str>{
    match itemset {
        "circles" => Ok(circle_set(&mut rng)),
        "squares" => Ok(square_set(&mut rng)),
        "shapes" => Ok(shapes_generate(&mut rng)),
        _ => Err("Invalid imageset")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delegator() {
        let mut rng = StdRng::new().expect("Failed to create RNG object");

        // test an itemset we know exists
        let result = imageset_delegator("squares", &mut rng);
        assert!(result.is_ok() && !result.is_err());

        let result = imageset_delegator("not-a-real-itemset",&mut rng);
        assert!(result.is_err() && !result.is_ok());

    }
}
