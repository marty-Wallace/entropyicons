
extern crate image;
extern crate rand;

use image::{GenericImage};
use image::png::PNGEncoder;
use std::path::Path;
use rand::{Rng, StdRng};

pub fn generate(itemset: &str, rng: &mut StdRng) -> Vec<u8>{

    let file = "image.png";
    let mut im  = image::open(&Path::new(&file)).unwrap();
    let file2 = "image2.png";
    let im2 = image::open(&Path::new(&file2)).unwrap();

    let (x1, y1) = im.dimensions();
    let (x2, y2) = im2.dimensions();

    let x3 = rng.gen::<u32>() % (x1-x2);
    let y3 = rng.gen::<u32>() % (y1-y2);

    im.copy_from(&im2, x3, y3);

    let mut x : Vec<u8> = vec![];
    PNGEncoder::new(&mut x)
        .encode(im.raw_pixels().as_slice(),
                im.width(),
                im.height(),
                im.color())
        .ok();
    x
}


#[cfg(test)]
mod tests {
    use super::*;
    use rand::{Rng, StdRng, SeedableRng};

    #[test]
    fn random_generate() {
        let mut init_rng = StdRng::new().expect("Failed to create rng");
        let mut bytes: [u8; 30] = [0_u8; 30];
        init_rng.fill_bytes(&mut bytes);
        let seed = bytes
            .iter()
            .map(|x| *x as usize)
            .collect::<Vec<_>>();
        let mut rng1 = StdRng::from_seed(&seed);
        let mut rng2 = StdRng::from_seed(&seed);
        //since itemset doesn't matter right now
        assert_eq!(generate("itemset", &mut rng1),
                   generate("itemset", &mut rng2));
    }
}
