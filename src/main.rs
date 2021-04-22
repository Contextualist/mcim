use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use pbr::ProgressBar;
use std::{
    fs::File,
    io::{BufWriter, Write},
};
use std::env;
use std::convert::TryInto;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        println!("Usage: ./mcim temperature mfield size step");
        std::process::exit(1);
    }
    let temp: f32 = args[1].parse().unwrap();
    let h: f32 = args[2].parse().unwrap();
    let l: usize = args[3].parse().unwrap();
    let step: usize = args[4].parse().unwrap();

    let n: i32 = (l * l) as i32;
    let mut m = vec![vec![1i32; l]; l];
    let mut energy = (-2 * n) as f32 + (-h * n as f32) ;
    let mut magn = n;

    let ofile = File::create("output.csv").unwrap();
    let mut writer = BufWriter::new(&ofile);
    let mut rng = SmallRng::from_entropy();
    let mut pb = ProgressBar::new(step.try_into().unwrap());
    pb.set_max_refresh_rate(Some(std::time::Duration::from_millis(100)));
    for _ in 0..step {
        for _ in 0..n {
            let rxy: usize = rng.gen_range(0..n).try_into().unwrap();
            let (x, y) = (rxy / l, rxy % l);
            let d_e = ((m[(x + l - 1) % l][y] + m[(x + 1) % l][y] + m[x][(y + l - 1) % l] + m[x][(y + 1) % l]) as f32
                + h) * m[x][y] as f32 * 2.0;
            if d_e <= 0.0 || rng.gen::<f32>() < (-d_e / temp).exp() {
                m[x][y] *= -1;
                energy += d_e;
                magn += 2 * m[x][y];
            }
        }
        writeln!(
            &mut writer, "{},{}",
            energy / (n as f32), (magn as f32) / (n as f32)
        )?;
        pb.inc();
    }
    Ok(())
}
