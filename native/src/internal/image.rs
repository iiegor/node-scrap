extern crate image;

use self::image::{ImageBuffer, Rgb};
use scrap::{Capturer, Display};
use std::io::ErrorKind::WouldBlock;
use std::path::Path;
use std::thread;
use std::time::Duration;

pub fn capture(path: &str) -> bool {
	let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;
    let path = Path::new(path);

    let display = Display::primary().expect("Couldn't find primary display.");
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
    let (w, h) = (capturer.width(), capturer.height());

    loop {
        let buffer = match capturer.frame() {
            Ok(buffer) => buffer,
            Err(error) => {
                if error.kind() == WouldBlock {
                    // Keep spinning.
                    thread::sleep(one_frame);
                    continue;
                } else {
                    panic!("Capture error: {}", error);
                }
            }
        };

        // The image crate doesn't support BGRA/BGR images yet.
        // See: https://github.com/PistonDevelopers/image/pull/666
        let mut bitflipped = Vec::with_capacity(w * h * 3);
        let stride = buffer.len() / h;

        for y in 0..h {
            for x in 0..w {
                let i = stride * y + 4 * x;
                bitflipped.extend_from_slice(&[
                    buffer[i + 2],
                    buffer[i + 1],
                    buffer[i],
                ]);
            }
        }

        let image: ImageBuffer<Rgb<u8>, _> =
            ImageBuffer::from_raw(
                w as u32,
                h as u32,
                bitflipped
            ).expect("Couldn't convert frame into image buffer.");

        image.save(&path).expect("Couldn't save image to `screenshot.png`.");
        break;
    }

	true
}
