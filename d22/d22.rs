#![feature(assert_matches)]

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

fn get_possible_spells(player: &Combatant) -> impl Iterator<Item = Spell> {
    [
        Spell::MagicMissile,
        Spell::Drain,
        Spell::Shield,
        Spell::Poison,
        Spell::Recharge,
    ]
    .into_iter()
    .filter(|spell| spell.cost() <= player.mana)
}

fn find_best_game(player: Combatant, boss: Combatant) -> (Vec<(GameState, Spell)>, GameState) {
    fn heuristic(state: &GameState) -> i64 {
        if state.player.hp <= 0 {
            return i64::MAX;
        }
        if state.boss.hp <= 0 {
            return 0;
        }
        (state.boss.hp * 10 - state.player.mana) as i64
    }

    fn distance(_: &GameState, edge: &Spell, _: &GameState) -> i64 {
        edge.cost() as i64
    }

    let start = GameState { player, boss };
    let (best_moves, end_state) = a_star_rev(
        &start,
        |state| state.boss.hp <= 0,
        |GameState { player, boss }| {
            get_possible_spells(player)
                .map(|spell| (game_turn(player.clone(), boss.clone(), spell), spell))
                .collect::<Vec<_>>()
        },
        heuristic,
        distance,
    )
    .unwrap();
    let best_moves = best_moves.into_iter().rev().collect();
    (best_moves, end_state)
}

fn main() {
    let player = Combatant::player();
    let boss = Combatant::boss();
    let (best_moves, end_state) = find_best_game(player, boss);
    let mana_used: i32 = best_moves.iter().map(|(_, s)| s.cost()).sum();
    for (state, spell) in best_moves {
        println!("{:?} | {:?}", state, spell);
    }
    println!("{:?}", end_state);
    println!("Mana used: {}", mana_used);
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
