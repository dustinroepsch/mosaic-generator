mod mosaic;
use crate::mosaic::Mosaic;
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

    /// The number of random samples to make
    #[structopt(long, default_value = "100")]
    num_random_samples: u32,

    /// The number of edge samples to make
    #[structopt(long, default_value = "100")]
    num_edge_samples: usize,

    #[structopt(short, default_value = "0.13")]
    low_threshold: f32,
    #[structopt(short, default_value = "0.15")]
    high_threshold: f32,
}

fn main() {
    let opt = Opt::from_args();
    let source_image = image::open(opt.input).unwrap();
    let mut mosaic = Mosaic::new(opt.num_random_samples, source_image);

    mosaic.add_detail_nodes(opt.num_edge_samples, opt.low_threshold, opt.high_threshold);

    let output_image = mosaic.render();
    output_image.save(opt.output).unwrap();
}
