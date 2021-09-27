use image::{DynamicImage, GenericImageView, Rgba};
use rand::prelude::*;
use rand::thread_rng;

/// A point in an image and the color at that point
#[derive(Clone, Copy)]
pub struct Node {
    /// The x coordinate in the source image
    pub x: u32,
    /// The y coordinate in the source image
    pub y: u32,
    /// The color at this point in the image
    pub color: Rgba<u8>,
}

impl Node {
    /// Returns a Vector of random Nodes in the given image
    ///
    /// # Arguments
    ///
    /// * `n` - The number of random nodes to return
    /// * `source_image` - The image to sample
    ///
    /// # Examples
    ///
    /// ```
    /// # use voronoi::mosaic::Node;
    /// # use image::DynamicImage;
    ///
    /// let img = DynamicImage::new_luma8(10, 10);
    /// let nodes = Node::random(10, &img);
    /// assert_eq!(nodes.len(), 10);
    /// ```
    pub fn random(n: u32, source_image: &DynamicImage) -> Vec<Node> {
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

    pub fn distance_squared(&self, x: u32, y: u32) -> i32 {
        let sx: i32 = self.x as i32;
        let sy: i32 = self.y as i32;
        let ox: i32 = x as i32;
        let oy: i32 = y as i32;
        (sx - ox) * (sx - ox) + (sy - oy) * (sy - oy)
    }
}
