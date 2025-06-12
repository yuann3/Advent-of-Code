use anyhow::Result;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    pub fn cost(&self) -> i32 {
        match self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }

    pub fn all_spells() -> Vec<Spell> {
        vec![
            Spell::MagicMissile,
            Spell::Drain,
            Spell::Shield,
            Spell::Poison,
            Spell::Recharge,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameState {
    // Player stats
    player_hp: i32,
    player_mana: i32,
    player_armor: i32,

    // Boss stats
    boss_hp: i32,
    boss_damage: i32,

    // Effect timers (0 means not active)
    shield_timer: i32,
    poison_timer: i32,
    recharge_timer: i32,

    // Game state
    player_turn: bool,
    total_mana_spent: i32,
}

impl GameState {
    pub fn new(boss_hp: i32, boss_damage: i32) -> Self {
        Self {
            player_hp: 50,
            player_mana: 500,
            player_armor: 0,
            boss_hp,
            boss_damage,
            shield_timer: 0,
            poison_timer: 0,
            recharge_timer: 0,
            player_turn: true,
            total_mana_spent: 0,
        }
    }

    pub fn apply_effects(&mut self) {
        self.player_armor = 0;

        if self.shield_timer > 0 {
            self.player_armor += 7;
            self.shield_timer -= 1;
        }

        if self.poison_timer > 0 {
            self.boss_hp -= 3;
            self.poison_timer -= 1;
        }

        if self.recharge_timer > 0 {
            self.player_mana += 101;
            self.recharge_timer -= 1;
        }
    }

    pub fn check_game_over(&self) -> Option<bool> {
        if self.boss_hp <= 0 {
            Some(true)
        } else if self.player_hp <= 0 {
            Some(false)
        } else {
            None
        }
    }

    pub fn can_cast_spell(&self, spell: Spell) -> bool {
        if self.player_mana < spell.cost() {
            return false;
        }

        match spell {
            Spell::Shield => self.shield_timer == 0,
            Spell::Poison => self.poison_timer == 0,
            Spell::Recharge => self.recharge_timer == 0,
            _ => true,
        }
    }

    pub fn cast_spell(&self, spell: Spell) -> Option<Self> {
        if !self.can_cast_spell(spell) {
            return None;
        }

        let mut new_state = self.clone();
        new_state.player_mana -= spell.cost();
        new_state.total_mana_spent += spell.cost();

        match spell {
            Spell::MagicMissile => {
                new_state.boss_hp -= 4;
            }
            Spell::Drain => {
                new_state.boss_hp -= 2;
                new_state.player_hp += 2;
            }
            Spell::Shield => {
                new_state.shield_timer = 6;
            }
            Spell::Poison => {
                new_state.poison_timer = 6;
            }
            Spell::Recharge => {
                new_state.recharge_timer = 5;
            }
        }

        Some(new_state)
    }

    pub fn boss_attack(&self) -> Self {
        let mut new_state = self.clone();

        let damage = std::cmp::max(1, self.boss_damage - self.player_armor);
        new_state.player_hp -= damage;
        new_state.player_turn = true;

        new_state
    }

    pub fn get_next_states(&self) -> Vec<Self> {
        let mut next_states = Vec::new();

        if self.player_turn {
            for spell in Spell::all_spells() {
                if let Some(mut new_state) = self.cast_spell(spell) {
                    new_state.player_turn = false;
                    next_states.push(new_state);
                }
            }
        } else {
            next_states.push(self.boss_attack());
        }

        next_states
    }
}

pub fn solve() -> Result<i32> {
    let boss_hp = 51;
    let boss_damage = 9;

    find_min_mana_to_win(boss_hp, boss_damage)
}

fn find_min_mana_to_win(boss_hp: i32, boss_damage: i32) -> Result<i32> {
    let initial_state = GameState::new(boss_hp, boss_damage);
    let mut queue = VecDeque::new();
    let mut seen = HashMap::new();

    queue.push_back(initial_state.clone());
    seen.insert(initial_state.clone(), 0);

    while let Some(current_state) = queue.pop_front() {
        let mut state_after_effects = current_state.clone();
        state_after_effects.apply_effects();

        if let Some(player_wins) = state_after_effects.check_game_over() {
            if player_wins {
                return Ok(state_after_effects.total_mana_spent);
            } else {
                continue;
            }
        }

        for next_state in state_after_effects.get_next_states() {
            let state_key = GameState {
                total_mana_spent: 0,
                ..next_state.clone()
            };

            if let Some(&prev_mana) = seen.get(&state_key) {
                if prev_mana <= next_state.total_mana_spent {
                    continue;
                }
            }

            seen.insert(state_key, next_state.total_mana_spent);
            queue.push_back(next_state);
        }
    }

    Err(anyhow::anyhow!("No winning strategy found"))
}
