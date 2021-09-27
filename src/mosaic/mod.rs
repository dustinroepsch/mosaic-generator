mod node;

pub use crate::mosaic::node::Node;
use image::{DynamicImage, GenericImageView, ImageBuffer, Luma, Rgba};
use imageproc::edges::canny;
use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rand::prelude::*;
use rand::thread_rng;
use rayon::prelude::*;

pub struct Mosaic {
    source_image: DynamicImage,
    nodes: Vec<Node>,
}

impl Mosaic {
    pub fn new(uniform_samples: u32, source: DynamicImage) -> Mosaic {
        assert!(uniform_samples != 0);

        Mosaic {
            nodes: Node::random(uniform_samples, &source),
            source_image: source,
        }
    }

    pub fn add_detail_nodes(&mut self, n: usize, low_threshold: f32, high_threshold: f32) {
        let edges = canny(&self.source_image.to_luma8(), low_threshold, high_threshold);
        edges.save("edges.png").unwrap();
        println!("Updated edges.png");
        let mut edge_nodes: Vec<Node> = Vec::new();
        for col in 0..edges.width() {
            for row in 0..edges.height() {
                if let Luma([255u8]) = *edges.get_pixel(col, row) {
                    edge_nodes.push(Node {
                        x: col,
                        y: row,
                        color: self.source_image.get_pixel(col, row),
                    });
                }
            }
        }
        let mut rng = thread_rng();
        for edge in edge_nodes.choose_multiple(&mut rng, n) {
            self.nodes.push(*edge);
        }
    }

    pub fn render(&self, should_draw_outline: bool) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let width = self.source_image.width();
        let height = self.source_image.height();

        let coords: Vec<(u32, u32)> = (0..width).cartesian_product(0..height).collect();

        let colors: Vec<Rgba<u8>> = coords
            .par_iter()
            .progress()
            .map(|(x, y)| {
                self.nodes
                    .iter()
                    .min_by_key(|&n| n.distance_squared(*x, *y))
                    .unwrap()
                    .color
            })
            .collect();

        image::ImageBuffer::from_fn(width, height, |x, y| {
            let original_color = colors[(x as u32 * height + y as u32) as usize];
            let black = Rgba([0, 0, 0, 255]);
            if should_draw_outline {
                match (-1..=1)
                    .combinations(2)
                    .map(|d| (x as i64 + d[0], y as i64 + d[1]))
                    .map(|(x, y)| (x as u32 * height + y as u32) as usize)
                    .filter(|&idx| idx < (width * height) as usize)
                    .map(|idx| colors[idx])
                    .all_equal()
                {
                    true => original_color,
                    false => black,
                }
            } else {
                original_color
            }
        })
    }
}
