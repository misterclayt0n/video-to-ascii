use opencv::prelude::*;
use opencv::videoio;
use opencv::imgproc;
use std::thread;
use std::time::Duration;

fn pixel_to_ascii(pixel_intensity: u8) -> char {
    let ascii_chars = "   ._-=+*!&#%$@";
    let index = (pixel_intensity as usize * ascii_chars.len()) / 256;
    ascii_chars.chars().nth(index).unwrap()
}

fn main() -> opencv::Result<()> {
    let video_path = "woman.mp4";
    let mut cap = videoio::VideoCapture::from_file(video_path, videoio::CAP_ANY)?;

    let fps = cap.get(videoio::CAP_PROP_FPS)?;
    println!("{}", fps);

    let frame_duration_ms = (1000.0 / fps) as u64;

    let width = 250;
    let mut height = 50;

    let frame_width = cap.get(videoio::CAP_PROP_FRAME_WIDTH)? as i32;
    let frame_height = cap.get(videoio::CAP_PROP_FRAME_HEIGHT)? as i32;
    println!("{} {}", frame_width, frame_height);

    height = ((width * frame_height) / frame_width) as i32 * 4194 / 10000;

    loop {
        let mut frame = Mat::default();
        cap.read(&mut frame)?;
        if frame.empty() {
            break;
        }

        let mut gray_frame = Mat::default();
        imgproc::cvt_color(&frame, &mut gray_frame, imgproc::COLOR_BGR2GRAY, 0)?;

        let mut resized_frame = Mat::default();
        imgproc::resize(
            &gray_frame,
            &mut resized_frame,
            opencv::core::Size::new(width, height),
            0.0,
            0.0,
            imgproc::INTER_LINEAR,
        )?;

        let mut ascii_frame = String::new();
        for i in 0..height {
            for j in 0..width {
                let pixel_intensity = *resized_frame.at_2d::<u8>(i, j)?;
                ascii_frame.push(pixel_to_ascii(pixel_intensity));
            }
            ascii_frame.push('\n');
        }

        // Limpar o terminal (funciona em ambientes Unix)
        print!("\x1B[2J\x1B[1;1H");
        println!("{}", ascii_frame);

        thread::sleep(Duration::from_millis(frame_duration_ms));
    }

    Ok(())
}
