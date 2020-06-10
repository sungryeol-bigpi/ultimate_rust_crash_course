// FINAL PROJECT
//
// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience: dyson.png and pens.png
// Feel free to use them or provide (or generate) your own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png
//
// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");

use clap::Clap;
use regex::Regex;

#[derive(Clap)]
#[clap(version = "1.0", author = "Sungryeol Park")]
struct Opts {
    #[clap(index = 1, required = true)]
    input: String,
    #[clap(index = 2, required = true, default_value = "no_value")]
    sigma: String,
    #[clap(short = "o", default_value = "no_value")]
    output: String,
    #[clap(short = "e", required = true)]
    effect: String,
    // #[clap(short = "f", default_value = "float_value")]
}

fn main() {
    let opts: Opts = Opts::parse();
    let infile = opts.input;
    let outfile = if opts.output == "no_value" {
        format!("out_{}", infile)
    } else {
        opts.output
    };
    let sigma = opts.sigma;
    println!("input {} output {} sigma {}", infile, outfile, sigma);

    // 1. First, you need to implement some basic command-line argument handling
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/
    // let mut args: Vec<String> = std::env::args().skip(1).collect();
    // if args.is_empty() {
    //     print_usage_and_exit();
    // }
    match opts.effect.as_str() {
        // EXAMPLE FOR CONVERSION OPERATIONS
        "blur" => {
            let sigma_f: f32 = if sigma == "no_value" {
                2.0
            } else {
                sigma.parse::<f32>().expect("sigma should be a float value")
            };
            blur(infile, outfile, sigma_f);
        }

        "brighten" => {
            let sigma_i: i32 = if sigma == "no_value" {
                100
            } else {
                sigma
                    .parse::<i32>()
                    .expect("sigma should be an integer value")
            };
            brighten(infile, outfile, sigma_i);
        }

        // **OPTION**
        "crop" => {
            let re =
                Regex::new(r"^(\d+),(\d+),(\d+),(\d+)$")
                    .expect("failed to compile regex");
            let caps = re.captures(sigma.as_str()).expect("sigma should be x/y/width/height");
            let x: u32 = caps.get(1).unwrap().as_str()
                .parse::<u32>()
                .expect("x should be unsigned integer");
            let y: u32 = caps.get(2).unwrap().as_str()
                .parse::<u32>()
                .expect("y should be unsigned integer");
            let width: u32 = caps.get(3).unwrap().as_str()
                .parse::<u32>()
                .expect("width should be unsigned integer");
            let height: u32 = caps.get(4).unwrap().as_str()
                .parse::<u32>()
                .expect("height should be unsigned integer");
            crop(infile, outfile, x, y, width, height);
        }
        // Crop -- see the crop() function below

        // **OPTION**
        // Rotate -- see the rotate() function below

        // **OPTION**
        // Invert -- see the invert() function below

        // **OPTION**
        // Grayscale -- see the grayscale() function below

        // A VERY DIFFERENT EXAMPLE...a really fun one. :-)
        "fractal" => {
            // if args.len() != 1 {
            //     print_usage_and_exit();
            // }
            // let outfile = args.remove(0);
            fractal(outfile);
        }

        // **OPTION**
        // Generate -- see the generate() function below -- this should be sort of like "fractal()"!

        // For everything else...
        _ => {
            print_usage_and_exit();
        }
    }
}

fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!("blur INFILE OUTFILE");
    // **OPTION**
    // Print useful information about what subcommands and arguments you can use
    // println!("...");
    std::process::exit(-1);
}

fn blur(infile: String, outfile: String, sigma: f32) {
    // Here's how you open an existing image file
    let img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.blur(sigma);
    // Here's how you save an image to a file.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn brighten(infile: String, outfile: String, sigma: i32) {
    // See blur() for an example of how to open / save an image.

    // .brighten() takes one argument, an i32.  Positive numbers brighten the
    // image. Negative numbers darken it.  It returns a new image.
    let img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.brighten(sigma);

    img2.save(outfile).expect("Failed writing OUTFILE");
}

fn crop(infile: String, outfile: String, x: u32, y: u32, width: u32, height: u32) {
    println!("cropping x {} y {} width {} height {}", x, y, width, height);
    println!("crop does not work at current version")
    // let img = image::open(infile).expect("failed to open Infile");
    // let img2 = img.crop(x, y, width, height);
    // img2.save(outfile).expect("failed writing outfile");
}

fn rotate(infile: String, outfile: String) {
    // See blur() for an example of how to open an image.

    // There are 3 rotate functions to choose from (all clockwise):
    //   .rotate90()
    //   .rotate180()
    //   .rotate270()
    // All three methods return a new image.  Pick one and use it!

    // Challenge: parse the rotation amount from the command-line, pass it
    // through to this function to select which method to call.

    // See blur() for an example of how to save the image.
}

fn invert(infile: String, outfile: String) {
    // See blur() for an example of how to open an image.

    // .invert() takes no arguments and converts the image in-place, so you
    // will use the same image to save out to a different file.

    // See blur() for an example of how to save the image.
}

fn grayscale(infile: String, outfile: String) {
    // See blur() for an example of how to open an image.

    // .grayscale() takes no arguments and converts the image in-place, so
    // you will use the same image to save out to a different file.

    // See blur() for an example of how to save the image.
}

fn generate(outfile: String) {
    // Create an ImageBuffer -- see fractal() for an example

    // Iterate over the coordinates and pixels of the image -- see fractal() for an example

    // Set the image to some solid color. -- see fractal() for an example

    // Challenge: parse some color data from the command-line, pass it through
    // to this function to use for the solid color.

    // Challenge 2: Generate something more interesting!

    // See blur() for an example of how to save the image
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
