use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use indicatif::ProgressBar;
use rand::{thread_rng, Rng};

struct Node {
    x: u32,
    y: u32,
    color: Rgba<u8>,
}

impl Node {
    fn random(n: u32, source_image: &DynamicImage) -> Vec<Node> {
        let mut rnd = thread_rng();

        assert!(n != 0);
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

    pub fn render(&self, display_progress: bool) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let pb = match display_progress {
            true => Some(ProgressBar::new(
                (self.source_image.width() * self.source_image.height()) as u64,
            )),
            false => None,
        };

        let img = image::ImageBuffer::from_fn(
            self.source_image.width(),
            self.source_image.height(),
            |x, y| {
                if let Some(pb) = &pb {
                    pb.inc(1);
                }
                self.uniform_nodes
                    .iter()
                    .min_by_key(|&n| n.distance_squared(x, y))
                    .unwrap()
                    .color
            },
        );

        if let Some(pb) = &pb {
            pb.finish();
        }

        img
    }
}
