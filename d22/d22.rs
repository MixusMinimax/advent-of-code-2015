use std::cmp;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum StatusEffect {
    Armor,
    Poisoned,
    Recharge,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
struct Combatant {
    hp: i32,
    damage: i32,
    mana: i32,
    status_effects: Vec<(i32, StatusEffect)>,
}

impl Combatant {
    fn player() -> Self {
        Combatant {
            hp: 100,
            mana: 200,
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

fn cast_spell(mut player: Combatant, mut boss: Combatant, spell: Spell) -> (Combatant, Combatant) {
    match spell {
        Spell::MagicMissile => {
            player.mana -= 53;
            boss.hp -= 4;
        }
        Spell::Drain => {
            player.mana -= 73;
            boss.hp -= 2;
            player.hp += 2;
        }
        Spell::Shield => player.status_effects.push((6, StatusEffect::Armor)),
        Spell::Poison => boss.status_effects.push((6, StatusEffect::Poisoned)),
        Spell::Recharge => player.status_effects.push((5, StatusEffect::Recharge)),
    }
    (player, boss)
}

fn attack_player(mut player: Combatant, boss: Combatant) -> (Combatant, Combatant) {
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
    (player, boss)
}

fn main() {}
