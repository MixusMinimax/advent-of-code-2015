use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;
use std::hash::Hash;
use std::{error, fmt};

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct NoPathFound;

impl fmt::Display for NoPathFound {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "no path found")
    }
}

impl error::Error for NoPathFound {}

/// returns the path in reverse order because it might be needed, and it would be inefficient to
/// reverse it twice in that case.
pub fn a_star_rev<Node, Edge, Neighbors>(
    start: &Node,
    goal: &Node,
    get_neighbors: impl Fn(&Node) -> Neighbors,
    heuristic: impl Fn(&Node) -> i64,
) -> Result<Vec<(Node, Edge)>, NoPathFound>
where
    Node: Clone + Eq + Hash,
    Edge: Clone + Copy,
    Neighbors: IntoIterator<Item = (Node, Edge)>,
{
    let mut open_set = HashSet::from([start.clone()]);
    let mut came_from = HashMap::<_, (Node, Edge)>::new();
    let mut g_score = HashMap::from([(start.clone(), 0i64)]);
    let mut f_score = HashMap::from([(start.clone(), heuristic(start))]);

    while let Some(current) = open_set
        .iter()
        .min_by_key(|&s| f_score.get(s).copied().unwrap_or(i64::MAX))
    {
        if current == goal {
            let mut total_path = Vec::new();
            let mut current = current;
            while came_from.contains_key(current) {
                let prev = came_from.get(current).unwrap();
                current = &prev.0;
                total_path.push(prev.clone());
            }
            return Ok(total_path);
        }

        let current = current.clone();
        open_set.remove(&current);

        for (neighbor, edge) in get_neighbors(&current) {
            let tentative_g_score = g_score.get(&current).copied().unwrap_or(i64::MAX);
            if tentative_g_score < g_score.get(&neighbor).copied().unwrap_or(i64::MAX) {
                came_from.insert(neighbor.clone(), (current.clone(), edge.clone()));
                g_score.insert(neighbor.clone(), tentative_g_score);
                f_score.insert(neighbor.clone(), tentative_g_score + heuristic(&neighbor));
                open_set.insert(neighbor);
            }
        }
    }

    Err(NoPathFound)
}
