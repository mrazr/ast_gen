#[macro_use]
use ast_gen::generate;
use structopt::StructOpt;
use std::path::PathBuf;

fn main() {
    let opt = Opt::from_args();
    // println!("{:?}", opt);
    let mut img = generate(opt.area, opt.intensities);
    
    if opt.smooth_param.is_some() {
        img = img.smoothen_all(opt.smooth_param.unwrap());
    }

    if opt.sigma.is_some() {
        img = img.blur_gray(opt.sigma.unwrap());
    }

    if let Some(p) = opt.fname_layers {
        img = img.save_layers(p.to_str().unwrap());
    }

    if let Some(p) = opt.fname_gray {
        img = img.save_gray(p.to_str().unwrap());
    }

    if let Some(p) = opt.fname_color {
        img = img.save_colored(p.to_str().unwrap());
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "ast_gen", about = "Generate image of a something like rock/asteroid")]
struct Opt {
    /// Size of the generated rock in number of pixels.
    area: u32,

    /// Number of intensities that will be present in the final image.
    #[structopt(short = "i", long = "intensities", default_value = "1")]
    intensities: usize,

    /// Smoothen "fuzziness" of the generated rock. Sensible values are between 1 and, say, 10,
    /// of course, depending on the size of the formation. Take a look at morphology operator
    /// dilation, which is used for the smoothing here.
    #[structopt(short = "s", long = "smoothen")]
    smooth_param: Option<u8>,

    /// Parameter for blurring by Gaussian filter.
    #[structopt(short = "b", long = "blur")]
    sigma: Option<f32>,

    /// Base name of layer images to write
    #[structopt(long = "wl", parse(from_os_str))]
    fname_layers: Option<PathBuf>,

    /// Name of the generated image in grayscale to write
    #[structopt(long = "wg", parse(from_os_str))]
    fname_gray: Option<PathBuf>,

    /// Name of the generated image in color to write
    #[structopt(long = "wc", parse(from_os_str))]
    fname_color: Option<PathBuf>,
}