use clap::{Arg, App};

pub fn get_app<'a>() -> App<'a,'a> {
    clap_app!(fractal =>
        (version: "1.0")
        (author: "A. Flores")
        (about: "VERY fast fractal generation at incredible hihg speed")
        (@arg width: -w --width +takes_value +required "Sets the width of the output image in pixels")
        (@arg height: -h --height +takes_value +required "Sets the height of the output image in pixels")
        (@arg real: -re --real +takes_value "Sets the position of the camera on the real axis (default: -1.6)")
        (@arg imaginary: -im --imag +takes_value "Sets the position of the camera on the imaginary axis (default: 0.0)")
        (@arg zoom: -z --zoom +takes_value "Sets the zoom level of the camera (default: -1.0)")
        (@arg file: -o --output-file +takes_value +required "The name of the file to output without extension (as a bmp)"))
}
