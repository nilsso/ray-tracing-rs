use std::path::Path;
use std::fs::File;
use std::io::Write;

fn encode_ppm() -> String {
    const IMAGE_W: u32 = 256;
    const IMAGE_H: u32 = 256;

    let mut res = format!("P3\n{} {}\n255\n", IMAGE_W, IMAGE_H).to_owned();
    for j in (0..IMAGE_H).rev() {
        for i in 0..IMAGE_W {
            let r = i as f32 / (IMAGE_W - 1) as f32;
            let g = j as f32 / (IMAGE_H - 1) as f32;
            let b = 0.25;

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;

            res.push_str(format!("{} {} {}\n", ir, ig, ib).as_str());
        }
    }
    res
}

fn main() -> std::io::Result<()> {
    let file_path = Path::new("test.ppm");
    let mut file = File::create(file_path)?;
    file.write_all(encode_ppm().as_bytes())?;
    Ok(())
}
