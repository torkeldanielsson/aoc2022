use parse_display::{Display, FromStr};
use std::{error::Error, fs};

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("Blueprint {i}: Each ore robot costs {ore_bot_ore} ore. Each clay robot costs {clay_bot_ore} ore. Each obsidian robot costs {obsidian_bot_ore} ore and {obsidian_bot_clay} clay. Each geode robot costs {geode_bot_ore} ore and {geode_bot_obsidian} obsidian.")]
struct Blueprint {
    i: i32,
    ore_bot_ore: i32,
    clay_bot_ore: i32,
    obsidian_bot_ore: i32,
    obsidian_bot_clay: i32,
    geode_bot_ore: i32,
    geode_bot_obsidian: i32,
}

#[derive(Clone, Debug)]
struct Resources {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Bots {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}

fn iterate(
    blueprint: &Blueprint,
    mut t: i32,
    mut resources: Resources,
    bots: Bots,
    max_geodes: &mut i32,
    mut history: String,
) {
    history = format!("{}== Minute {} ==\n", history, t);

    resources.ore += bots.ore;
    resources.clay += bots.clay;
    resources.obsidian += bots.obsidian;
    resources.geode += bots.geode;

    history = format!("{}{} ore => {}\n", history, bots.ore, resources.ore);
    if bots.clay > 0 {
        history = format!("{}{} clay => {}\n", history, bots.clay, resources.clay);
    }
    if bots.obsidian > 0 {
        history = format!(
            "{}{} obsidian => {}\n",
            history, bots.obsidian, resources.obsidian
        );
    }
    if bots.geode > 0 {
        history = format!("{}{} geode => {}\n", history, bots.geode, resources.geode);
    }

    if resources.geode > *max_geodes {
        *max_geodes = resources.geode;

        println!("############### {}", max_geodes);
        println!("{history}");
        println!();
    }

    if t >= 24 {
        return;
    }
    t += 1;

    for combo in [
        Bots {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
        Bots {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
        Bots {
            ore: 0,
            clay: 1,
            obsidian: 0,
            geode: 0,
        },
        Bots {
            ore: 0,
            clay: 0,
            obsidian: 1,
            geode: 0,
        },
        Bots {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 1,
        },
    ] {
        let mut new_bots = bots.clone();
        new_bots.ore += combo.ore;
        new_bots.clay += combo.clay;
        new_bots.obsidian += combo.obsidian;
        new_bots.geode += combo.geode;

        let mut new_resources = resources.clone();
        new_resources.ore -= combo.ore * blueprint.ore_bot_ore;
        new_resources.ore -= combo.clay * blueprint.clay_bot_ore;
        new_resources.ore -= combo.obsidian * blueprint.obsidian_bot_ore;
        new_resources.clay -= combo.obsidian * blueprint.obsidian_bot_clay;
        new_resources.ore -= combo.geode * blueprint.geode_bot_ore;
        new_resources.obsidian -= combo.geode * blueprint.geode_bot_obsidian;

        if new_resources.ore >= 0
            && new_resources.clay >= 0
            && new_resources.obsidian >= 0
            && new_resources.geode >= 0
        {
            iterate(
                blueprint,
                t,
                new_resources,
                new_bots,
                max_geodes,
                format!("{}\n{:?}\n", history, combo),
            );
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    for blueprint in fs::read_to_string("test")?
        .lines()
        .map(|s| s.parse::<Blueprint>().unwrap())
    {
        let mut max_geodes = 0;
        iterate(
            &blueprint,
            0,
            Resources {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            Bots {
                ore: 1,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            &mut max_geodes,
            String::new(),
        );
        println!("{}: {}", blueprint.i, max_geodes);
    }

    Ok(())
}
