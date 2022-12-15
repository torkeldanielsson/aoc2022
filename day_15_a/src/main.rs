use parse_display::{Display, FromStr};
use std::{error::Error, fs};

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("Sensor at x={sx}, y={sy}: closest beacon is at x={bx}, y={by}")]
struct Sensor {
    sx: i32,
    sy: i32,
    bx: i32,
    by: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let sensors = fs::read_to_string("input")?
        .lines()
        .map(|s| s.parse::<Sensor>().unwrap())
        .collect::<Vec<_>>();

    let mut covered = std::collections::HashSet::new();
    let mut beacons = std::collections::HashSet::new();
    let y = 2000000;

    for s in &sensors {
        let dist = (s.bx - s.sx).abs() + (s.by - s.sy).abs();
        let half_span = dist - (s.sy - y).abs();
        if half_span > 0 {
            for x in s.sx - half_span..=s.sx + half_span {
                covered.insert(x);
            }
        }
        if s.by == y {
            beacons.insert(s.bx);
        }
    }

    println!("{}", covered.len() - beacons.len());

    Ok(())
}
