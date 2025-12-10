use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};
use std::hash::Hash;

/// Breadth-First Search - finds shortest path from start to goal.
/// Returns Some(path) if goal is reachable, None otherwise.
pub fn bfs<N, FN, IN>(
    start: N,
    mut neighbors: FN,
    goal: impl Fn(&N) -> bool,
) -> Option<Vec<N>>
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
{
    let mut queue = VecDeque::new();
    let mut visited = HashSet::default();
    let mut parents: HashMap<N, N> = HashMap::default();

    queue.push_back(start.clone());
    visited.insert(start.clone());

    while let Some(current) = queue.pop_front() {
        if goal(&current) {
            // Reconstruct path
            let mut path = vec![current.clone()];
            let mut node = current;
            while let Some(parent) = parents.get(&node) {
                path.push(parent.clone());
                node = parent.clone();
            }
            path.reverse();
            return Some(path);
        }

        for neighbor in neighbors(&current) {
            if visited.insert(neighbor.clone()) {
                parents.insert(neighbor.clone(), current.clone());
                queue.push_back(neighbor);
            }
        }
    }

    None
}

/// BFS that returns all reachable nodes with their distances from start.
pub fn bfs_all_reachable<N, FN, IN>(start: N, mut neighbors: FN) -> HashMap<N, usize>
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
{
    let mut distances = HashMap::default();
    let mut queue = VecDeque::new();

    distances.insert(start.clone(), 0);
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        let current_dist = *distances.get(&current).unwrap();

        for neighbor in neighbors(&current) {
            if !distances.contains_key(&neighbor) {
                distances.insert(neighbor.clone(), current_dist + 1);
                queue.push_back(neighbor);
            }
        }
    }

    distances
}

/// Dijkstra's algorithm - finds shortest path with weighted edges.
/// Returns Some((path, cost)) if goal is reachable, None otherwise.
pub fn dijkstra<N, FN, IN>(
    start: N,
    mut neighbors: FN,
    goal: impl Fn(&N) -> bool,
) -> Option<(Vec<N>, usize)>
where
    N: Eq + Hash + Clone + Ord,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, usize)>,
{
    let mut heap = BinaryHeap::new();
    let mut costs: HashMap<N, usize> = HashMap::default();
    let mut parents: HashMap<N, N> = HashMap::default();

    heap.push(Reverse((0, start.clone())));
    costs.insert(start.clone(), 0);

    while let Some(Reverse((cost, current))) = heap.pop() {
        if goal(&current) {
            // Reconstruct path
            let mut path = vec![current.clone()];
            let mut node = current;
            while let Some(parent) = parents.get(&node) {
                path.push(parent.clone());
                node = parent.clone();
            }
            path.reverse();
            return Some((path, cost));
        }

        // Skip if we've found a better path already
        if cost > *costs.get(&current).unwrap_or(&usize::MAX) {
            continue;
        }

        for (neighbor, edge_cost) in neighbors(&current) {
            let new_cost = cost + edge_cost;
            let is_better = new_cost < *costs.get(&neighbor).unwrap_or(&usize::MAX);

            if is_better {
                costs.insert(neighbor.clone(), new_cost);
                parents.insert(neighbor.clone(), current.clone());
                heap.push(Reverse((new_cost, neighbor)));
            }
        }
    }

    None
}

/// A* search - finds shortest path using a heuristic.
/// Returns Some((path, cost)) if goal is reachable, None otherwise.
pub fn astar<N, FN, IN, FH>(
    start: N,
    mut neighbors: FN,
    heuristic: FH,
    goal: impl Fn(&N) -> bool,
) -> Option<(Vec<N>, usize)>
where
    N: Eq + Hash + Clone + Ord,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, usize)>,
    FH: Fn(&N) -> usize,
{
    let mut heap = BinaryHeap::new();
    let mut costs: HashMap<N, usize> = HashMap::default();
    let mut parents: HashMap<N, N> = HashMap::default();

    let start_h = heuristic(&start);
    heap.push(Reverse((start_h, 0, start.clone())));
    costs.insert(start.clone(), 0);

    while let Some(Reverse((_, cost, current))) = heap.pop() {
        if goal(&current) {
            // Reconstruct path
            let mut path = vec![current.clone()];
            let mut node = current;
            while let Some(parent) = parents.get(&node) {
                path.push(parent.clone());
                node = parent.clone();
            }
            path.reverse();
            return Some((path, cost));
        }

        // Skip if we've found a better path already
        if cost > *costs.get(&current).unwrap_or(&usize::MAX) {
            continue;
        }

        for (neighbor, edge_cost) in neighbors(&current) {
            let new_cost = cost + edge_cost;
            let is_better = new_cost < *costs.get(&neighbor).unwrap_or(&usize::MAX);

            if is_better {
                costs.insert(neighbor.clone(), new_cost);
                parents.insert(neighbor.clone(), current.clone());
                let f_score = new_cost + heuristic(&neighbor);
                heap.push(Reverse((f_score, new_cost, neighbor)));
            }
        }
    }

    None
}

/// DFS to visit all reachable nodes.
/// Returns a set of all reachable nodes from start.
pub fn dfs_visit_all<N, FN, IN>(start: N, mut neighbors: FN) -> HashSet<N>
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
{
    let mut visited = HashSet::default();
    let mut stack = vec![start];

    while let Some(current) = stack.pop() {
        if visited.insert(current.clone()) {
            for neighbor in neighbors(&current) {
                if !visited.contains(&neighbor) {
                    stack.push(neighbor);
                }
            }
        }
    }

    visited
}

#[cfg(test)]
mod tests {
    use super::*;

    fn simple_graph_neighbors(node: &i32) -> Vec<i32> {
        match node {
            0 => vec![1, 2],
            1 => vec![3],
            2 => vec![3],
            3 => vec![4],
            _ => vec![],
        }
    }

    fn weighted_graph_neighbors(node: &i32) -> Vec<(i32, usize)> {
        match node {
            0 => vec![(1, 1), (2, 4)],
            1 => vec![(3, 2)],
            2 => vec![(3, 1)],
            3 => vec![(4, 1)],
            _ => vec![],
        }
    }

    #[test]
    fn test_bfs_finds_path() {
        let path = bfs(0, simple_graph_neighbors, |&n| n == 4);
        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(path.first(), Some(&0));
        assert_eq!(path.last(), Some(&4));
    }

    #[test]
    fn test_bfs_no_path() {
        let path = bfs(0, simple_graph_neighbors, |&n| n == 10);
        assert!(path.is_none());
    }

    #[test]
    fn test_bfs_all_reachable() {
        let distances = bfs_all_reachable(0, simple_graph_neighbors);
        assert_eq!(distances.get(&0), Some(&0));
        assert_eq!(distances.get(&1), Some(&1));
        assert_eq!(distances.get(&4), Some(&3));
    }

    #[test]
    fn test_dijkstra_finds_shortest_path() {
        let result = dijkstra(0, weighted_graph_neighbors, |&n| n == 4);
        assert!(result.is_some());
        let (path, cost) = result.unwrap();
        assert_eq!(path.first(), Some(&0));
        assert_eq!(path.last(), Some(&4));
        assert_eq!(cost, 4); // 0->1->3->4 = 1+2+1 = 4
    }

    #[test]
    fn test_dijkstra_no_path() {
        let result = dijkstra(0, weighted_graph_neighbors, |&n| n == 10);
        assert!(result.is_none());
    }

    #[test]
    fn test_astar_finds_path() {
        // Simple heuristic: distance to goal
        let heuristic = |&n: &i32| (4 - n).abs() as usize;
        let result = astar(0, weighted_graph_neighbors, heuristic, |&n| n == 4);
        assert!(result.is_some());
        let (path, cost) = result.unwrap();
        assert_eq!(path.first(), Some(&0));
        assert_eq!(path.last(), Some(&4));
        assert_eq!(cost, 4); // 0->1->3->4 = 1+2+1 = 4
    }

    #[test]
    fn test_dfs_visit_all() {
        let visited = dfs_visit_all(0, simple_graph_neighbors);
        assert_eq!(visited.len(), 5);
        assert!(visited.contains(&0));
        assert!(visited.contains(&4));
    }

    #[test]
    fn test_bfs_on_grid() {
        // Simple 3x3 grid pathfinding test
        let neighbors = |&(x, y): &(i32, i32)| {
            vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
                .into_iter()
                .filter(|&(nx, ny)| nx >= 0 && ny >= 0 && nx < 3 && ny < 3)
        };

        let path = bfs((0, 0), neighbors, |&pos| pos == (2, 2));
        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(path.first(), Some(&(0, 0)));
        assert_eq!(path.last(), Some(&(2, 2)));
        assert_eq!(path.len(), 5); // Shortest path length
    }
}
