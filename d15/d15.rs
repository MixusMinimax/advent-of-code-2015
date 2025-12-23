use aoc2015::compositions;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, char, space0, space1};
use nom::combinator::{map, opt};
use nom::multi::fold;
use std::cmp;
use std::iter::zip;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq, Default)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl FromStr for Ingredient {
    type Err = nom::Err<nom::error::Error<String>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn properties(s: &str) -> IResult<&str, Ingredient> {
            fold(
                0..,
                (
                    alt((
                        tag("capacity"),
                        tag("durability"),
                        tag("flavor"),
                        tag("texture"),
                        tag("calories"),
                    )),
                    space1,
                    nom::character::complete::i32,
                    opt(char(',')),
                    space0,
                ),
                Ingredient::default,
                |ingredient: Ingredient, (prop, _, v, ..)| match prop {
                    "capacity" => Ingredient {
                        capacity: v,
                        ..ingredient
                    },
                    "durability" => Ingredient {
                        durability: v,
                        ..ingredient
                    },
                    "flavor" => Ingredient {
                        flavor: v,
                        ..ingredient
                    },
                    "texture" => Ingredient {
                        texture: v,
                        ..ingredient
                    },
                    "calories" => Ingredient {
                        calories: v,
                        ..ingredient
                    },
                    _ => ingredient,
                },
            )
            .parse(s)
        }

        map(
            (alphanumeric1, char(':'), space0, properties),
            |(name, _, _, props)| Ingredient {
                name: name.to_string(),
                ..props
            },
        )
        .parse(s)
        .map(|(_, o)| o)
        .map_err(<nom::Err<nom::error::Error<&str>>>::to_owned)
    }
}

fn cookie_score(ingredients: &[Ingredient], amounts: &[u32]) -> i64 {
    let capacity: i32 = zip(ingredients, amounts)
        .map(|(i, &a)| i.capacity * a as i32)
        .sum();
    let durability: i32 = zip(ingredients, amounts)
        .map(|(i, &a)| i.durability * a as i32)
        .sum();
    let flavor: i32 = zip(ingredients, amounts)
        .map(|(i, &a)| i.flavor * a as i32)
        .sum();
    let texture: i32 = zip(ingredients, amounts)
        .map(|(i, &a)| i.texture * a as i32)
        .sum();
    let capacity = cmp::max(capacity, 0);
    let durability = cmp::max(durability, 0);
    let flavor = cmp::max(flavor, 0);
    let texture = cmp::max(texture, 0);
    capacity as i64 * durability as i64 * flavor as i64 * texture as i64
}

fn main() {
    let input = include_str!("input.txt");
    let ingredients: Vec<Ingredient> = input
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();
    let best_score = compositions(ingredients.len() as u32, 100)
        .map(|c| cookie_score(&ingredients, &c))
        .max()
        .unwrap();
    println!("Part1: {}", best_score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            "Butterscotch:".parse(),
            Ok(Ingredient {
                name: "Butterscotch".to_string(),
                ..Default::default()
            })
        );
        assert_eq!(
            "Butterscotch: capacity -1".parse(),
            Ok(Ingredient {
                name: "Butterscotch".to_string(),
                capacity: -1,
                ..Default::default()
            })
        );
        assert_eq!(
            "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3".parse(),
            Ok(Ingredient {
                name: "Cinnamon".to_string(),
                capacity: 2,
                durability: 3,
                flavor: -2,
                texture: -1,
                calories: 3,
            })
        );
    }

    #[test]
    fn test_cookie_score() {
        let ingredients = vec![
            Ingredient {
                name: "Butterscotch".to_string(),
                capacity: -1,
                durability: -2,
                flavor: 6,
                texture: 3,
                calories: 8,
            },
            Ingredient {
                name: "Cinnamon".to_string(),
                capacity: 2,
                durability: 3,
                flavor: -2,
                texture: -1,
                calories: 3,
            },
        ];
        let score = cookie_score(&ingredients, &[44, 56]);
        assert_eq!(score, 62842880);
    }
}
