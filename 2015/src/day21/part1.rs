use anyhow::Result;
use aoc_lib::read_lines;

pub fn solve() -> Result<u32> {
    // Hit Points: XX
    // Damage:     YY
    // Armor:      ZZ
    let lines = read_lines("input/day21.in")?;
    let mut iter = lines.iter();
    let boss_hp: u32 = iter
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .parse()?;
    let boss_dmg: u32 = iter
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .parse()?;
    let boss_arm: u32 = iter
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .parse()?;

    // (cost, damage, armor)
    let weapons = vec![
        (8, 4, 0),
        (10, 5, 0),
        (25, 6, 0),
        (40, 7, 0),
        (74, 8, 0),
    ];

    let armors = vec![
        (0, 0, 0),
        (13, 0, 1),
        (31, 0, 2),
        (53, 0, 3),
        (75, 0, 4),
        (102, 0, 5),
    ];

    let rings = vec![
        (0, 0, 0),
        (0, 0, 0),
        (25, 1, 0),
        (50, 2, 0),
        (100, 3, 0),
        (20, 0, 1),
        (40, 0, 2),
        (80, 0, 3),
    ];

    let player_hp = 100;
    let mut best_cost = u32::MAX;

    for &(wc, wd, wa) in &weapons {
        for &(ac, ad, aa) in &armors {
            for i in 0..rings.len() {
                for j in i + 1..rings.len() {
                    let (rc1, rd1, ra1) = rings[i];
                    let (rc2, rd2, ra2) = rings[j];

                    let total_cost = wc + ac + rc1 + rc2;
                    let total_dmg: u32 = wd + ad + rd1 + rd2;
                    let total_arm = wa + aa + ra1 + ra2;

                    let dmg_to_boss = (total_dmg.saturating_sub(boss_arm)).max(1);
                    let dmg_to_player = (boss_dmg.saturating_sub(total_arm)).max(1);

                    let turns_to_kill_boss =
                        (boss_hp + dmg_to_boss - 1) / dmg_to_boss;
                    let turns_to_kill_player =
                        (player_hp + dmg_to_player - 1) / dmg_to_player;

                    if turns_to_kill_boss <= turns_to_kill_player {
                        best_cost = best_cost.min(total_cost);
                    }
                }
            }
        }
    }

    Ok(best_cost)
}
