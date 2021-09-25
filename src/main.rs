use image::{GenericImageView, Rgba};
use indicatif::ProgressBar;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "voronoi", about = "make voronoi")]
struct Opt {
    /// The path to the input image
    input: PathBuf,

    /// The destination of the output image
    #[structopt(short, default_value = "out.png")]
    output: PathBuf,

    /// The number of samples to make
    #[structopt(long, default_value = "100")]
    num_samples: u32,
}

struct Node {
    x: u32,
    y: u32,
    color: Rgba<u8>,
}

impl Node {
    fn distance_squared(&self, x: u32, y: u32) -> u32 {
        (self.x - x) * (self.x - x) + (self.y - y) * (self.y - y)
    }
}

fn main() {
    let opt = Opt::from_args();
    let source_image = image::open(opt.input).unwrap();

    let nodes: Vec<Node> = (0..opt.num_samples)
        .map(|i| {
            let n_x = (i * 1312311) % source_image.width();
            let n_y = (i * 171344566) % source_image.height();
            Node {
                x: n_x,
                y: n_y,
                color: source_image.get_pixel(n_x, n_y),
            }
        })
        .collect();

    let bar = ProgressBar::new((source_image.width() * source_image.height()) as u64);

    let output_image =
        image::ImageBuffer::from_fn(source_image.width(), source_image.height(), |x, y| {
            bar.inc(1);
            nodes
                .iter()
                .min_by_key(|&n| n.distance_squared(x, y))
                .unwrap()
                .color
        });
    bar.finish();

    output_image.save(opt.output).unwrap();
}
