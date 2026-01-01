#![feature(assert_matches)]

use crate::printout::print_turn;
use aoc2015::graph::a_star_rev;
use std::cmp;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum StatusEffect {
    Armor,
    Poisoned,
    Recharge,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Spell {
    /// Costs 53 mana. Deals 4 damage.
    MagicMissile,
    /// Costs 73 mana. Deals 2 damage and heals 2 hit points.
    Drain,
    /// Costs 113 mana. Adds 7 armor for 6 turns.
    Shield,
    /// Costs 173 mana. Deals 3 damage per turn for 6 turns.
    Poison,
    /// Costs 229 mana. Gives 101 mana per turn for 5 turns.
    Recharge,
}

impl Spell {
    fn cost(&self) -> i32 {
        match self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
struct Combatant {
    hp: i32,
    damage: i32,
    mana: i32,
    status_effects: Vec<(i32, StatusEffect)>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
struct GameState {
    player: Combatant,
    boss: Combatant,
}

impl Combatant {
    fn player() -> Self {
        Combatant {
            hp: 50,
            mana: 500,
            ..Combatant::default()
        }
    }

    fn boss() -> Self {
        let config: HashMap<&str, i32> = serde_yaml::from_str(include_str!("boss.txt")).unwrap();
        Combatant {
            hp: config["Hit Points"],
            damage: config["Damage"],
            ..Combatant::default()
        }
    }
}

fn apply_effects(mut combatant: Combatant) -> Combatant {
    for (ttl, effect) in combatant.status_effects.iter_mut() {
        match effect {
            StatusEffect::Armor => {}
            StatusEffect::Poisoned => combatant.hp -= 3,
            StatusEffect::Recharge => combatant.mana += 101,
        }
        *ttl -= 1;
    }
    combatant.status_effects.retain(|(ttl, _)| *ttl > 0);
    combatant
}

fn cast_spell(mut player: Combatant, mut boss: Combatant, spell: Spell) -> GameState {
    player.mana -= spell.cost();
    match spell {
        Spell::MagicMissile => {
            boss.hp -= 4;
        }
        Spell::Drain => {
            boss.hp -= 2;
            player.hp += 2;
        }
        Spell::Shield => player.status_effects.push((6, StatusEffect::Armor)),
        Spell::Poison => boss.status_effects.push((6, StatusEffect::Poisoned)),
        Spell::Recharge => player.status_effects.push((5, StatusEffect::Recharge)),
    }
    GameState { player, boss }
}

fn attack_player(mut player: Combatant, boss: Combatant) -> GameState {
    if boss.hp <= 0 {
        return GameState { player, boss };
    }
    let armor = if player
        .status_effects
        .iter()
        .any(|(_, effect)| *effect == StatusEffect::Armor)
    {
        7
    } else {
        0
    };
    player.hp -= cmp::max(1, boss.damage - armor);
    GameState { player, boss }
}

fn game_turn(player: Combatant, boss: Combatant, spell: Spell) -> GameState {
    let player = apply_effects(player);
    let boss = apply_effects(boss);
    let GameState { player, boss } = cast_spell(player, boss, spell);
    let player = apply_effects(player);
    let boss = apply_effects(boss);
    let GameState { player, boss } = attack_player(player, boss);
    GameState { player, boss }
}

fn get_possible_spells(player: &Combatant, boss: &Combatant) -> impl IntoIterator<Item = Spell> {
    let player_after_effects = apply_effects(player.clone());
    let boss_after_effects = apply_effects(boss.clone());
    [
        Spell::MagicMissile,
        Spell::Drain,
        Spell::Shield,
        Spell::Poison,
        Spell::Recharge,
    ]
    .into_iter()
    .filter(|spell| spell.cost() <= player_after_effects.mana)
    .filter(|spell| match spell {
        Spell::Shield => !player_after_effects
            .status_effects
            .iter()
            .any(|(_, e)| *e == StatusEffect::Armor),
        Spell::Poison => !boss_after_effects
            .status_effects
            .iter()
            .any(|(_, e)| *e == StatusEffect::Poisoned),
        Spell::Recharge => !player_after_effects
            .status_effects
            .iter()
            .any(|(_, e)| *e == StatusEffect::Recharge),
        _ => true,
    })
    .filter(|spell| {
        let GameState { player: p, .. } = game_turn(player.clone(), boss.clone(), *spell);
        p.hp > 0
    })
    .collect::<Vec<_>>()
}

fn find_best_game(player: Combatant, boss: Combatant) -> (Vec<(GameState, Spell)>, GameState) {
    fn is_goal(state: &GameState) -> bool {
        state.boss.hp <= 0
    }

    fn get_neighbors(GameState { player, boss }: &GameState) -> Vec<(GameState, Spell)> {
        get_possible_spells(player, boss)
            .into_iter()
            .map(|spell| (game_turn(player.clone(), boss.clone(), spell), spell))
            .collect::<Vec<_>>()
    }

    fn heuristic(state: &GameState) -> i64 {
        if state.player.hp <= 0 {
            return i64::MAX;
        }
        if state.boss.hp <= 0 {
            return 0;
        }
        (state.boss.hp * 10 - state.player.mana) as i64
    }

    let distance = |_: &GameState, edge: &Spell, _: &GameState| -> i64 { edge.cost() as i64 };

    let start = GameState { player, boss };
    let (best_moves, end_state) =
        a_star_rev(&start, is_goal, get_neighbors, heuristic, distance).unwrap();
    let best_moves = best_moves.into_iter().rev().collect();
    (best_moves, end_state)
}

mod printout {
    use super::*;

    fn print_stats_and_effects(game_state: &GameState) {
        println!(
            "- Player has {} hit points, {} armor, {} mana",
            game_state.player.hp,
            if game_state
                .player
                .status_effects
                .iter()
                .any(|(_, e)| *e == StatusEffect::Armor)
            {
                7
            } else {
                0
            },
            game_state.player.mana
        );
        println!("- Boss has {} hit points", game_state.boss.hp);
        for (ttl, effect) in game_state
            .player
            .status_effects
            .iter()
            .chain(game_state.boss.status_effects.iter())
        {
            match effect {
                StatusEffect::Armor => println!("Shield's timer is now {}.", ttl),
                StatusEffect::Poisoned => {
                    println!("Poison deals 3 damage; its timer is now {}.", ttl)
                }
                StatusEffect::Recharge => {
                    println!("Recharge provides 101 mana; its timer is now {}.", ttl)
                }
            }
            if *ttl == 0 {
                match effect {
                    StatusEffect::Armor => println!("Shield wears off, decreasing armor by 7."),
                    StatusEffect::Poisoned => println!("Poison wears off."),
                    StatusEffect::Recharge => println!("Recharge wears off."),
                }
            }
        }
    }

    pub fn print_turn(game_state: GameState, spell: Spell) {
        println!("-- Player turn --");
        print_stats_and_effects(&game_state);
        match spell {
            Spell::MagicMissile => println!("Player casts Magic Missile, dealing 4 damage."),
            Spell::Drain => {
                println!("Player casts Drain, dealing 2 damage, and healing 2 hit points.")
            }
            Spell::Shield => println!("Player casts Shield, increasing armor by 7."),
            Spell::Poison => println!("Player casts Poison."),
            Spell::Recharge => println!("Player casts Recharge."),
        };
        println!();
        let game_state = {
            let player = apply_effects(game_state.player);
            let boss = apply_effects(game_state.boss);
            cast_spell(player, boss, spell)
        };
        println!("-- Boss turn --");
        print_stats_and_effects(&game_state);
        let game_state = {
            let player = apply_effects(game_state.player);
            let boss = apply_effects(game_state.boss);
            attack_player(player, boss)
        };
        if game_state.boss.hp <= 0 {
            println!("Boss is dead.");
        } else {
            println!("Boss attacks for {} damage.", game_state.boss.damage);
        }
    }
}

fn main() {
    let player = Combatant::player();
    let boss = Combatant::boss();
    let (best_moves, _) = find_best_game(player, boss);
    let mana_used: i32 = best_moves.iter().map(|(_, s)| s.cost()).sum();
    println!("Mana used: {}", mana_used);
    for (state, spell) in best_moves {
        print_turn(state, spell);
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches::assert_matches;

    #[test]
    fn test_apply_effects() {
        let mut combatant = Combatant {
            hp: 10,
            mana: 0,
            status_effects: vec![(2, StatusEffect::Poisoned), (1, StatusEffect::Recharge)],
            ..Combatant::default()
        };
        combatant = apply_effects(combatant);
        assert_eq!(combatant.hp, 7);
        assert_eq!(combatant.mana, 101);
        assert_eq!(combatant.status_effects.len(), 1);
        assert_eq!(combatant.status_effects[0].0, 1);
        assert_eq!(combatant.status_effects[0].1, StatusEffect::Poisoned);
    }

    #[test]
    fn test_cast_spell() {
        let player = Combatant {
            hp: 10,
            mana: 200,
            ..Combatant::default()
        };
        let boss = Combatant {
            hp: 14,
            ..Combatant::default()
        };
        let GameState { player, boss } = cast_spell(player, boss, Spell::MagicMissile);
        assert_eq!(player.mana, 147);
        assert_eq!(boss.hp, 10);
    }

    #[test]
    fn test_attack_player() {
        let player = Combatant {
            hp: 10,
            ..Combatant::default()
        };
        let boss = Combatant {
            damage: 8,
            ..Combatant::default()
        };
        let GameState { player, .. } = attack_player(player, boss);
        assert_eq!(player.hp, 2);
    }

    #[test]
    fn test_game_turn() {
        let player = Combatant {
            hp: 10,
            mana: 250,
            ..Combatant::default()
        };
        let boss = Combatant {
            hp: 13,
            damage: 8,
            ..Combatant::default()
        };
        // Player casts Poison
        let GameState { player, boss } = game_turn(player, boss, Spell::Poison);
        assert_eq!(player.hp, 2);
        assert_eq!(player.mana, 77);
        assert_eq!(boss.hp, 10);
        // Player casts Magic Missile
        let GameState { player, boss } = game_turn(player, boss, Spell::MagicMissile);
        assert_eq!(player.hp, 2);
        assert_eq!(player.mana, 24);
        assert_eq!(boss.hp, 0);
    }

    #[test]
    fn test_best_game_1() {
        let player = Combatant {
            hp: 10,
            mana: 250,
            ..Combatant::default()
        };
        let boss = Combatant {
            hp: 13,
            damage: 8,
            ..Combatant::default()
        };
        let (best_moves, end_state) = find_best_game(player, boss);
        let mana_used: i32 = best_moves.iter().map(|(_, s)| s.cost()).sum();
        assert_eq!(mana_used, 173 + 53);
        assert_eq!(
            best_moves.iter().map(|(_, s)| *s).collect::<Vec<_>>(),
            [Spell::Poison, Spell::MagicMissile]
        );
        assert_matches!(
            end_state,
            GameState {
                player: Combatant {
                    hp: 2,
                    mana: 24,
                    ..
                },
                boss: Combatant { hp: 0, .. }
            }
        );
    }

    #[test]
    fn test_best_game_2() {
        let player = Combatant {
            hp: 10,
            mana: 250,
            ..Combatant::default()
        };
        let boss = Combatant {
            hp: 14,
            damage: 8,
            ..Combatant::default()
        };
        let (best_moves, end_state) = find_best_game(player, boss);
        let mana_used: i32 = best_moves.iter().map(|(_, s)| s.cost()).sum();
        assert_eq!(mana_used, 229 + 113 + 73 + 173 + 53);
        assert_eq!(
            best_moves.iter().map(|(_, s)| *s).collect::<Vec<_>>(),
            [
                Spell::Recharge,
                Spell::Shield,
                Spell::Drain,
                Spell::Poison,
                Spell::MagicMissile
            ]
        );
        assert_matches!(
            end_state,
            GameState {
                player: Combatant {
                    hp: 1,
                    mana: 114,
                    ..
                },
                boss: Combatant { hp: -1, .. },
            }
        );
    }
}
