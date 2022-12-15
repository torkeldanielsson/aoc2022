use parse_display::{Display, FromStr};
use std::{error::Error, fs};

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("Sensor at x={sx}, y={sy}: closest beacon is at x={bx}, y={by}")]
struct RawSensor {
    sx: i64,
    sy: i64,
    bx: i64,
    by: i64,
}

#[derive(Display, PartialEq, Debug)]
#[display("Sensor x={x}, y={y}, dist={dist}")]
struct Sensor {
    x: i64,
    y: i64,
    dist: i64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let sensors = fs::read_to_string("input")?
        .lines()
        .map(|s| s.parse::<RawSensor>().unwrap())
        .map(|rs| Sensor {
            x: rs.sx,
            y: rs.sy,
            dist: (rs.bx - rs.sx).abs() + (rs.by - rs.sy).abs(),
        })
        .collect::<Vec<_>>();

    let bounds = 4000000;

    for y in 0..=bounds {
        let mut x = 0;
        loop {
            let x_before = x;
            for s in &sensors {
                let half_span = s.dist - (s.y - y).abs();
                if half_span >= 0 && s.x - half_span <= x && x < s.x + half_span + 1 {
                    x = s.x + half_span + 1;
                }
            }
            if x >= bounds {
                break;
            }
            if x_before == x {
                println!("{}", x * 4000000 + y);
                return Ok(());
            }
        }
    }

    Ok(())
}
