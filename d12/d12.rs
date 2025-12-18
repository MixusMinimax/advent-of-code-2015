#![feature(assert_matches)]

use serde::de::{Error, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct NumberSum(i64);

impl<'de> Deserialize<'de> for NumberSum {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        struct NumberSumVisitor;

        impl<'de> Visitor<'de> for NumberSumVisitor {
            type Value = NumberSum;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                write!(formatter, "piss off")
            }

            fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(NumberSum(0))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(NumberSum(v))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(NumberSum(v as i64))
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(NumberSum(v as i64))
            }

            fn visit_char<E>(self, _: char) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(NumberSum(0))
            }

            fn visit_str<E>(self, _: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(NumberSum(0))
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(NumberSum(0))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut sum: i64 = 0;
                while let Some(NumberSum(i)) = seq.next_element()? {
                    sum += i;
                }
                Ok(NumberSum(sum))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut sum: i64 = 0;
                while let Some((_, NumberSum(i))) = map.next_entry::<&str, _>()? {
                    sum += i;
                }
                Ok(NumberSum(sum))
            }
        }

        deserializer.deserialize_any(NumberSumVisitor)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct NumberSumRed(i64, bool);

impl<'de> Deserialize<'de> for NumberSumRed {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        struct NumberSumVisitor;

        impl<'de> Visitor<'de> for NumberSumVisitor {
            type Value = NumberSumRed;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                write!(formatter, "piss off")
            }

            fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(NumberSumRed(0, false))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(NumberSumRed(v, false))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(NumberSumRed(v as i64, false))
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(NumberSumRed(v as i64, false))
            }

            fn visit_char<E>(self, _: char) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(NumberSumRed(0, false))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(NumberSumRed(0, v == "red"))
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(NumberSumRed(0, false))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut sum: i64 = 0;
                while let Some(NumberSumRed(i, _)) = seq.next_element()? {
                    sum += i;
                }
                Ok(NumberSumRed(sum, false))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut sum: i64 = 0;
                let mut red = false;
                while let Some((_, NumberSumRed(i, b))) = map.next_entry::<&str, _>()? {
                    if b {
                        red = true;
                    }
                    sum += i;
                }
                Ok(NumberSumRed(if red { 0 } else { sum }, false))
            }
        }

        deserializer.deserialize_any(NumberSumVisitor)
    }
}

fn main() {
    let s = include_str!("input.txt");
    let result: NumberSum = serde_json::from_str(s).unwrap();
    println!("Part1: {}", result.0);
    let result: NumberSumRed = serde_json::from_str(s).unwrap();
    println!("Part2: {}", result.0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches::assert_matches;

    #[test]
    fn test_part1() {
        use serde_json::from_str;
        assert_matches!(from_str(r#"[1,2,3]"#).unwrap(), NumberSum(6));
        assert_matches!(from_str(r#"{"a":2,"b":4}"#).unwrap(), NumberSum(6));
        assert_matches!(from_str(r#"[[[3]]]"#).unwrap(), NumberSum(3));
        assert_matches!(from_str(r#"{"a":{"b":4},"c":-1}"#).unwrap(), NumberSum(3));
        assert_matches!(from_str(r#"{"a":[-1,1]}"#).unwrap(), NumberSum(0));
        assert_matches!(from_str(r#"[-1,{"a":1}]"#).unwrap(), NumberSum(0));
        assert_matches!(from_str(r#"[]"#).unwrap(), NumberSum(0));
        assert_matches!(from_str(r#"{}"#).unwrap(), NumberSum(0));
    }

    #[test]
    fn test_part2() {
        use serde_json::from_str;
        assert_matches!(from_str(r#"[1,2,3]"#).unwrap(), NumberSumRed(6, _));
        assert_matches!(
            from_str(r#"[1,{"c":"red","b":2},3]"#).unwrap(),
            NumberSumRed(4, _)
        );
        assert_matches!(
            from_str(r#"{"d":"red","e":[1,2,3,4],"f":5}"#).unwrap(),
            NumberSumRed(0, _)
        );
        assert_matches!(from_str(r#"[1,"red",5]"#).unwrap(), NumberSumRed(6, _));
    }
}
