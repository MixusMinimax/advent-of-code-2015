use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Item {
    name: String,
    cost: i32,
    damage: i32,
    armor: i32,
}

impl FromStr for Item {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // language=regexp
        lazy_static! {
            static ref pat: Regex = Regex::new(r#"^(.*?\S)\s+(\d+)\s+(\d+)\s+(\d+)\s*$"#).unwrap();
        }
        || -> Option<Self> {
            let (_, [name, cost, damage, armor]) = pat.captures(s)?.extract();
            Some(Item {
                name: name.to_string(),
                cost: cost.parse().ok()?,
                damage: damage.parse().ok()?,
                armor: armor.parse().ok()?,
            })
        }()
        .ok_or(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
struct Combatant {
    hp: i32,
    damage: i32,
    armor: i32,
}

impl FromStr for Combatant {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        || -> Option<Self> {
            let mut result = Combatant::default();
            for line in s.lines() {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                let mut tokens = line.split(':');
                let prop = tokens.next()?.trim();
                let v: i32 = tokens.next()?.trim().parse().ok()?;
                match prop.to_lowercase().as_str() {
                    "hit points" => result.hp = v,
                    "damage" => result.damage = v,
                    "armor" => result.armor = v,
                    _ => return None,
                }
            }
            Some(result)
        }()
        .ok_or(())
    }
}

impl Combatant {
    fn with_items(self, items: &[Item]) -> Self {
        let (damage, armor) = items
            .iter()
            .fold((self.damage, self.armor), |(damage, armor), item| {
                (damage + item.damage, armor + item.armor)
            });
        Self {
            hp: self.hp,
            damage,
            armor,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
struct Items {
    weapons: Vec<Item>,
    armor: Vec<Item>,
    rings: Vec<Item>,
}

impl FromStr for Items {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        || -> Option<Self> {
            // language=regexp
            lazy_static! {
                static ref header: Regex = Regex::new("^([^:]+):").unwrap();
            }

            enum State {
                Begin,
                Weapons,
                Armor,
                Rings,
            }
            let mut state = State::Begin;
            let mut items = Items::default();

            for line in s.lines() {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                if let Some(h) = header.captures(line) {
                    match h.get(1)?.as_str() {
                        "Weapons" => state = State::Weapons,
                        "Armor" => state = State::Armor,
                        "Rings" => state = State::Rings,
                        _ => return None,
                    }
                    continue;
                }
                let item = line.parse().ok()?;
                match state {
                    State::Begin => return None,
                    State::Weapons => items.weapons.push(item),
                    State::Armor => items.armor.push(item),
                    State::Rings => items.rings.push(item),
                }
            }
            if let State::Begin = state {
                return None;
            }
            Some(items)
        }()
        .ok_or(())
    }
}

fn possible_combinations(items: &Items) -> Vec<Vec<Item>> {
    let mut result = Vec::new();
    for weapon in items.weapons.iter() {
        for armor in [None].into_iter().chain(items.armor.iter().map(Some)) {
            result.push(
                [Some(weapon), armor]
                    .into_iter()
                    .flatten()
                    .cloned()
                    .collect(),
            );
            for (ring_1_index, ring_1) in items.rings.iter().enumerate() {
                for ring_2 in [None]
                    .into_iter()
                    .chain(items.rings.iter().skip(ring_1_index + 1).map(Some))
                {
                    result.push(
                        [Some(weapon), armor, Some(ring_1), ring_2]
                            .into_iter()
                            .flatten()
                            .cloned()
                            .collect(),
                    );
                }
            }
        }
    }
    result
}

fn attack(attacker: &Combatant, target: &mut Combatant) {
    let damage = cmp::max(1, attacker.damage - target.armor);
    target.hp -= damage;
}

fn player_wins(mut player: Combatant, mut boss: Combatant) -> bool {
    let mut player_turn = true;
    loop {
        if player_turn {
            attack(&player, &mut boss);
            if boss.hp <= 0 {
                return true;
            }
        } else {
            attack(&boss, &mut player);
            if player.hp <= 0 {
                return false;
            }
        }
        player_turn = !player_turn;
    }
}

fn main() {
    let items: Items = include_str!("items.txt").parse().unwrap();
    let combos = possible_combinations(&items);
    let player: Combatant = include_str!("player.txt").parse().unwrap();
    let boss: Combatant = include_str!("boss.txt").parse().unwrap();
    let mut smallest_cost = i32::MAX;
    let mut best_loadout = Vec::new();
    for combo in combos.iter() {
        let cost: i32 = combo.iter().map(|c| c.cost).sum();
        if cost < smallest_cost && player_wins(player.clone().with_items(combo), boss.clone()) {
            smallest_cost = cost;
            best_loadout = combo.iter().collect();
        }
    }
    println!("Part1: {}, loadout:\n{:?}", smallest_cost, best_loadout);

    let mut biggest_cost = 0;
    let mut worst_loadout = Vec::new();
    for combo in combos.iter() {
        let cost: i32 = combo.iter().map(|c| c.cost).sum();
        if cost > biggest_cost && !player_wins(player.clone().with_items(combo), boss.clone()) {
            biggest_cost = cost;
            worst_loadout = combo.iter().collect();
        }
    }
    println!("Part2: {}, loadout:\n{:?}", biggest_cost, worst_loadout);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            "Dagger        8     4       0".parse(),
            Ok(Item {
                name: "Dagger".to_string(),
                cost: 8,
                damage: 4,
                armor: 0,
            })
        );
        assert_eq!(
            "Damage +1    25     1       0".parse(),
            Ok(Item {
                name: "Damage +1".to_string(),
                cost: 25,
                damage: 1,
                armor: 0,
            })
        );
    }

    #[test]
    fn test_stats() {
        let expected = Combatant {
            hp: 100,
            damage: 6 + 2,
            armor: 4,
        };
        let actual = Combatant {
            hp: 100,
            ..Combatant::default()
        }
        .with_items(
            &[
                "Warhammer    25     6       0",
                "Bandedmail   75     0       4",
                "Damage +2    50     2       0",
            ]
            .into_iter()
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()
            .unwrap(),
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_items() {
        let expected = Items {
            weapons: vec![
                Item {
                    name: "Dagger".to_string(),
                    cost: 8,
                    damage: 4,
                    armor: 0,
                },
                Item {
                    name: "Shortsword".to_string(),
                    cost: 10,
                    damage: 5,
                    armor: 0,
                },
                Item {
                    name: "Warhammer".to_string(),
                    cost: 25,
                    damage: 6,
                    armor: 0,
                },
                Item {
                    name: "Longsword".to_string(),
                    cost: 40,
                    damage: 7,
                    armor: 0,
                },
                Item {
                    name: "Greataxe".to_string(),
                    cost: 74,
                    damage: 8,
                    armor: 0,
                },
            ],
            armor: vec![
                Item {
                    name: "Leather".to_string(),
                    cost: 13,
                    damage: 0,
                    armor: 1,
                },
                Item {
                    name: "Chainmail".to_string(),
                    cost: 31,
                    damage: 0,
                    armor: 2,
                },
                Item {
                    name: "Splintmail".to_string(),
                    cost: 53,
                    damage: 0,
                    armor: 3,
                },
                Item {
                    name: "Bandedmail".to_string(),
                    cost: 75,
                    damage: 0,
                    armor: 4,
                },
                Item {
                    name: "Platemail".to_string(),
                    cost: 102,
                    damage: 0,
                    armor: 5,
                },
            ],
            rings: vec![
                Item {
                    name: "Damage +1".to_string(),
                    cost: 25,
                    damage: 1,
                    armor: 0,
                },
                Item {
                    name: "Damage +2".to_string(),
                    cost: 50,
                    damage: 2,
                    armor: 0,
                },
                Item {
                    name: "Damage +3".to_string(),
                    cost: 100,
                    damage: 3,
                    armor: 0,
                },
                Item {
                    name: "Defense +1".to_string(),
                    cost: 20,
                    damage: 0,
                    armor: 1,
                },
                Item {
                    name: "Defense +2".to_string(),
                    cost: 40,
                    damage: 0,
                    armor: 2,
                },
                Item {
                    name: "Defense +3".to_string(),
                    cost: 80,
                    damage: 0,
                    armor: 3,
                },
            ],
        };
        let actual: Result<Items, _> = r#"
                Weapons:    Cost  Damage  Armor
                Dagger        8     4       0
                Shortsword   10     5       0
                Warhammer    25     6       0
                Longsword    40     7       0
                Greataxe     74     8       0

                Armor:      Cost  Damage  Armor
                Leather      13     0       1
                Chainmail    31     0       2
                Splintmail   53     0       3
                Bandedmail   75     0       4
                Platemail   102     0       5

                Rings:      Cost  Damage  Armor
                Damage +1    25     1       0
                Damage +2    50     2       0
                Damage +3   100     3       0
                Defense +1   20     0       1
                Defense +2   40     0       2
                Defense +3   80     0       3"#
            .parse();
        assert_eq!(actual, Ok(expected));
    }

    #[test]
    fn test_parse_combatant() {
        let expected = Combatant {
            hp: 100,
            damage: 5,
            armor: 69,
        };
        let actual: Result<Combatant, _> = "Hit Points: 100\nDamage: 5\nArmor: 69\n".parse();
        assert_eq!(actual, Ok(expected));
    }

    #[test]
    fn test_player_wins() {
        let player = Combatant {
            hp: 8,
            damage: 5,
            armor: 5,
        };
        let boss = Combatant {
            hp: 12,
            damage: 7,
            armor: 2,
        };
        assert!(player_wins(player, boss));
    }
}
