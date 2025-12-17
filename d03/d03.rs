use std::collections::HashSet;
use vecmath::Vector2;

fn direction(ins: char) -> Vector2<i32> {
    match ins {
        '<' => [-1, 0],
        '>' => [1, 0],
        'v' => [0, 1],
        '^' => [0, -1],
        _ => [0, 0],
    }
}

fn visited_houses(instructions: &str) -> HashSet<Vector2<i32>> {
    instructions
        .chars()
        .fold(
            (HashSet::from([[0, 0]]), [0, 0]),
            |(mut visited, mut pos), ins| {
                pos = vecmath::vec2_add(pos, direction(ins));
                visited.insert(pos);
                (visited, pos)
            },
        )
        .0
}

fn visited_robo_houses(instructions: &str) -> HashSet<Vector2<i32>> {
    struct State {
        santa_visited: HashSet<Vector2<i32>>,
        robo_visited: HashSet<Vector2<i32>>,
        santa_pos: Vector2<i32>,
        robo_pos: Vector2<i32>,
    }

    let result = instructions.chars().enumerate().fold(
        State {
            santa_visited: HashSet::from([[0, 0]]),
            robo_visited: HashSet::from([[0, 0]]),
            santa_pos: [0, 0],
            robo_pos: [0, 0],
        },
        |mut state, (index, ins)| {
            if index % 2 == 0 {
                state.santa_pos = vecmath::vec2_add(state.santa_pos, direction(ins));
                state.santa_visited.insert(state.santa_pos);
            } else {
                state.robo_pos = vecmath::vec2_add(state.robo_pos, direction(ins));
                state.robo_visited.insert(state.robo_pos);
            }
            state
        },
    );
    result
        .santa_visited
        .union(&result.robo_visited)
        .copied()
        .collect()
}

fn main() {
    let input = include_str!("input.txt");
    let visit_count = visited_houses(input).len();
    println!("Part1: {}", visit_count);
    let robo_visit_count = visited_robo_houses(input).len();
    println!("Part2: {}", robo_visit_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visited_houses() {
        assert_eq!(visited_houses(">"), HashSet::from([[0, 0], [1, 0]]));
        assert_eq!(
            visited_houses("^>v<"),
            HashSet::from([[0, 0], [0, -1], [1, -1], [1, 0]])
        );
        assert_eq!(
            visited_houses("^v^v^v^v^v"),
            HashSet::from([[0, 0], [0, -1]])
        );
    }

    #[test]
    fn test_visited_robo_houses() {
        assert_eq!(visited_robo_houses(">"), HashSet::from([[0, 0], [1, 0]]));
        assert_eq!(
            visited_robo_houses("^>v<"),
            HashSet::from([[0, 0], [0, -1], [1, 0]])
        );
        assert_eq!(
            visited_robo_houses("^v^v^v^v^v"),
            HashSet::from([
                [0, 0],
                [0, 1],
                [0, 2],
                [0, 3],
                [0, 4],
                [0, 5],
                [0, -1],
                [0, -2],
                [0, -3],
                [0, -4],
                [0, -5]
            ])
        );
    }
}
