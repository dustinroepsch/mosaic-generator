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

    /// The number of samples to make
    #[structopt(long, default_value = "100")]
    num_samples: u32,
}

fn main() {
    let opt = Opt::from_args();
    let source_image = image::open(opt.input).unwrap();
    let mosaic = Mosaic::new(opt.num_samples, source_image);

    let output_image = mosaic.render();
    output_image.save(opt.output).unwrap();
}
