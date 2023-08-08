use std::fs::File;
use std::io::BufWriter;
use std::process;
use librsvg::*;
use cairo;

pub fn convert(path: &str) {
    let handle = Loader::new().read_path(path).expect("Couldn't load the file");

    let surface = cairo::ImageSurface::create(cairo::Format::ARgb32, 512, 512).unwrap();
    let cr = cairo::Context::new(&surface).expect("Failed to create a cairo context");

    let renderer = CairoRenderer::new(&handle);
    let res = renderer.render_document(&cr, &cairo::Rectangle {
        x: 0.0,
        y: 0.0,
        width: f64::from(512),
        height: f64::from(512),
    });

    match res {
        Ok(()) => {
            let output_path = path.replace(".svg", ".png");
            let mut file = BufWriter::new(File::create(output_path).unwrap());

            surface.write_to_png(&mut file).unwrap();
        }

        Err(e) => {
            eprintln!("rendering error: {}", e);
            process::exit(1);
        }
    }

}