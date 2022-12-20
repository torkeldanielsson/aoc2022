use parse_display::{Display, FromStr};
use std::sync::{Arc, Mutex};
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
    max_geodes: Arc<Mutex<std::collections::HashMap<i32, i32>>>,
    best_geodes_at_timestep: &mut std::collections::HashMap<i32, i32>,
) {
    t += 1;

    if *best_geodes_at_timestep.get(&t).unwrap_or(&0) > resources.geode {
        return;
    }
    best_geodes_at_timestep.insert(t, resources.geode);

    if t >= 24 {
        resources.ore += bots.ore;
        resources.clay += bots.clay;
        resources.obsidian += bots.obsidian;
        resources.geode += bots.geode;

        if let Ok(mut lock) = max_geodes.lock() {
            if resources.geode > *lock.get(&blueprint.i).unwrap_or(&0) {
                lock.insert(blueprint.i, resources.geode);
            }
        }

        return;
    }

    let mut possible_combos = Vec::new();

    for combo in [
        Bots {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 1,
        },
        Bots {
            ore: 0,
            clay: 0,
            obsidian: 1,
            geode: 0,
        },
        Bots {
            ore: 0,
            clay: 1,
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
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
    ] {
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
            let mut new_bots = bots.clone();
            new_bots.ore += combo.ore;
            new_bots.clay += combo.clay;
            new_bots.obsidian += combo.obsidian;
            new_bots.geode += combo.geode;

            new_resources.ore += bots.ore;
            new_resources.clay += bots.clay;
            new_resources.obsidian += bots.obsidian;
            new_resources.geode += bots.geode;

            possible_combos.push((new_resources, new_bots));

            if combo.geode > 1 {
                break;
            }
        }
    }

    for combo in possible_combos {
        iterate(
            blueprint,
            t,
            combo.0,
            combo.1,
            max_geodes.clone(),
            best_geodes_at_timestep,
        );
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut threads = Vec::new();
    let max_geodes = Arc::new(Mutex::new(std::collections::HashMap::new()));
    for blueprint in fs::read_to_string("input")?
        .lines()
        .map(|s| s.parse::<Blueprint>().unwrap())
    {
        let local_max_geodes = max_geodes.clone();
        threads.push(std::thread::spawn(move || {
            let mut best_geodes_at_timestep = std::collections::HashMap::new();
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
                local_max_geodes,
                &mut best_geodes_at_timestep,
            );
        }));
    }

    for t in threads {
        t.join().expect("Couldn't join on the associated thread");
    }

    if let Ok(lock) = max_geodes.lock() {
        println!("{:?}", *lock);

        let mut res = 0;
        for v in &*lock {
            res += v.0 * v.1;
        }
        println!("res: {res}");
    }

    // 1070 too low

    Ok(())
}
