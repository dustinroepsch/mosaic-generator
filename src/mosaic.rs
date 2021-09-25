use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rand::{thread_rng, Rng};
use rayon::prelude::*;

struct Node {
    x: u32,
    y: u32,
    color: Rgba<u8>,
}

impl Node {
    fn random(n: u32, source_image: &DynamicImage) -> Vec<Node> {
        assert!(n != 0);

        let mut rnd = thread_rng();

        (0..n)
            .map(|_| {
                let n_x = rnd.gen_range(0..source_image.width());
                let n_y = rnd.gen_range(0..source_image.height());
                Node {
                    x: n_x,
                    y: n_y,
                    color: source_image.get_pixel(n_x, n_y),
                }
            })
            .collect()
    }

    fn distance_squared(&self, x: u32, y: u32) -> u32 {
        (self.x - x) * (self.x - x) + (self.y - y) * (self.y - y)
    }
}

pub struct Mosaic {
    source_image: DynamicImage,
    uniform_nodes: Vec<Node>,
    //    detail_nodes: Vec<Node>,
}

impl Mosaic {
    pub fn new(uniform_samples: u32, source: DynamicImage) -> Mosaic {
        assert!(uniform_samples != 0);

        Mosaic {
            uniform_nodes: Node::random(uniform_samples, &source),
            source_image: source,
            //           detail_nodes: Vec::new(),
        }
    }

    pub fn render(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let width = self.source_image.width();
        let height = self.source_image.height();

        let coords: Vec<(u32, u32)> = (0..width).cartesian_product(0..height).collect();

        let colors: Vec<Rgba<u8>> = coords
            .par_iter()
            .progress()
            .map(|(x, y)| {
                self.uniform_nodes
                    .iter()
                    .min_by_key(|&n| n.distance_squared(*x, *y))
                    .unwrap()
                    .color
            })
            .collect();

        image::ImageBuffer::from_fn(width, height, |x, y| {
            match (-1..=1)
                .combinations(2)
                .map(|d| (x as i64 + d[0], y as i64 + d[1]))
                .map(|(x, y)| (x as u32 * height + y as u32) as usize)
                .filter(|&idx| idx < (width * height) as usize)
                .map(|idx| colors[idx])
                .all_equal()
            {
                true => colors[(x as u32 * height + y as u32) as usize],
                false => Rgba([0, 0, 0, 255]),
            }
        })
    }
}