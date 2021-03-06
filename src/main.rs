use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use pbr::ProgressBar;
use std::{
    fs::File,
    io::{BufWriter, Write},
};
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        println!("Usage: ./mcim temperature mfield size step");
        std::process::exit(1);
    }
    let temp: f32 = args[1].parse().expect("temperature should be a real number");
    let h: f32 = args[2].parse().expect("magnetic field should be a real number");
    let l: usize = args[3].parse().expect("size should be an integer");
    let step: usize = args[4].parse().expect("step should be an integer");

    let n: i32 = (l * l) as i32;
    let mut m = vec![vec![1i32; l]; l];
    let mut energy = (-2 * n) as f32 + (-h * n as f32) ;
    let mut magn = n;

    let ofile = File::create("output.csv").expect("unable to write to current directory");
    let mut writer = BufWriter::new(&ofile);
    let mut rng = SmallRng::from_entropy();
    for x in 0..l {
        for y in 0..l {
            if rng.gen::<f32>() < 0.5 {
                m[x][y] = -1;
                energy += ((m[(x + l - 1) % l][y] + m[(x + 1) % l][y] + m[x][(y + l - 1) % l] + m[x][(y + 1) % l]) as f32
                + h) * m[x][y] as f32 * 2.0;
                magn += 2 * m[x][y];
            }
        }
    }
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
