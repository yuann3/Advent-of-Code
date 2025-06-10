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
    let mut worst_cost = 0;

    for &(wc, wd, wa) in &weapons {
        for &(ac, ad, aa) in &armors {
            for i in 0..rings.len() {
                for j in i + 1..rings.len() {
                    let (rc1, rd1, ra1) = rings[i];
                    let (rc2, rd2, ra2) = rings[j];

                    let cost = wc + ac + rc1 + rc2;
                    let dmg: u32 = wd + ad + rd1 + rd2;
                    let arm  = wa + aa + ra1 + ra2;

                    let dmg_to_boss   = (dmg.saturating_sub(boss_arm)).max(1);
                    let dmg_to_player = (boss_dmg.saturating_sub(arm)).max(1);

                    let t_boss   = (boss_hp   + dmg_to_boss   - 1) / dmg_to_boss;
                    let t_player = (player_hp + dmg_to_player - 1) / dmg_to_player;

                    if t_boss > t_player {
                    worst_cost = worst_cost.max(cost);
                    }
                }
            }
        }
    }

    Ok(worst_cost)
}
