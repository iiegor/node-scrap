use scrap::{Capturer, Display};
use std::io::Write;
use std::io::ErrorKind::WouldBlock;
use std::process::{Command, Stdio};

pub fn record(path: &str, fps: &str) {
    let display = Display::primary().unwrap();
    let (w, h) = (display.width(), display.height());

    let child = Command::new("ffmpeg")
        .args(&[
            "-y",
            "-f", "rawvideo",
            "-pixel_format", "bgr0",
            "-video_size", &format!("{}x{}", w, h),
            "-framerate", fps,
            "-vcodec", "rawvideo",
            "-i", "-",
            path,
        ])
        .stdin(Stdio::piped())
        .spawn()
        .expect("This example requires ffplay.");

    let mut capturer = Capturer::new(display).unwrap();
    let mut out = child.stdin.unwrap();

    loop {
        match capturer.frame() {
            Ok(frame) => {
                // Write the frame, removing end-of-row padding.
                let stride = frame.len() / h;
                let rowlen = 4 * w;
                for row in frame.chunks(stride) {
                    let row = &row[..rowlen];
                    out.write_all(row).unwrap();
                }
            },
            Err(ref e) if e.kind() == WouldBlock => {
                // Wait for the frame.
            }
            Err(_) => {
                // We're done here.
                break;
            }
        }
    }
}
