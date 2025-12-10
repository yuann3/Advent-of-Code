# AOC-Lib Expansion Design

**Date:** 2025-12-10
**Status:** Approved
**Scope:** Standard toolkit covering 80% of AoC use cases

## Overview

Expand aoc-lib from a minimal I/O library to a comprehensive Advent of Code toolkit. Add four new modules: grid utilities, mathematical helpers, pathfinding algorithms, and parsing utilities. Follow TDD approach with comprehensive edge case testing.

## Architecture

### Library Structure

```
aoc-lib/src/
├── lib.rs           # Top-level exports + existing I/O
├── benchmark.rs     # Existing benchmarking (unchanged)
├── grid.rs          # 2D grid utilities
├── math.rs          # Mathematical helpers
├── pathfinding.rs   # Graph algorithms
└── parsing.rs       # Input parsing utilities
```

### Module Philosophy

Each module is self-contained with minimal cross-dependencies. This keeps compile times fast and allows importing only what's needed.

### Top-Level Re-exports

```rust
// Most common items exported at top level for convenience
pub use grid::Grid;
pub use math::{gcd, lcm};
pub use pathfinding::{bfs, dijkstra};
pub use parsing::{extract_numbers, extract_ints};

// Modules available for specialized access
pub mod grid;
pub mod math;
pub mod pathfinding;
pub mod parsing;
```

### New Dependencies

Add to `Cargo.toml`:
```toml
regex = "1"           # For parsing utilities
num-traits = "0.2"    # For generic numeric operations
rustc-hash = "2"      # Faster HashMap/HashSet for competitive programming
```

## Module 1: Grid

### Core Type

```rust
pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}
```

### Construction

- `Grid::new(width, height, default: T)` - Create filled grid
- `Grid::from_vec(data: Vec<T>, width: usize)` - From flat vector
- `Grid::parse<F>(lines: &[String], f: F)` - Parse from string lines with converter function
- `Grid::parse_chars(lines: &[String])` - Shorthand for character grids

### Core Operations

- `get(x, y) -> Option<&T>` / `get_mut(x, y) -> Option<&mut T>` - Bounds-checked access
- `[index]` operator using `(x, y)` tuples - Panics on out of bounds
- `contains(x, y) -> bool` - Check if coordinates valid
- `rows()` / `cols()` - Iterators over rows/columns
- `iter()` / `iter_mut()` - Iterate all cells with coordinates

### Direction Handling

```rust
pub enum Direction { North, South, East, West }
pub const CARDINAL: [Direction; 4] = [North, South, East, West];
pub const DIAGONAL: [(isize, isize); 4] = [(-1,-1), (-1,1), (1,-1), (1,1)];
pub const ALL_8: [(isize, isize); 8] = [...];
```

### Neighbor Methods

- `neighbors4(x, y) -> impl Iterator<Item=(usize, usize, &T)>` - Cardinal neighbors
- `neighbors8(x, y)` - All 8 directions
- `neighbor(x, y, dir: Direction) -> Option<(usize, usize, &T)>` - Specific direction

### Design Decisions

- Uses `(usize, usize)` coordinates for simplicity in AoC context
- Signed offsets handled via `isize` when computing neighbors
- Iterator-based APIs for composability

## Module 2: Math

### Basic Number Theory

```rust
pub fn gcd<T: Integer>(a: T, b: T) -> T
pub fn lcm<T: Integer>(a: T, b: T) -> T
pub fn gcd_extended(a: i64, b: i64) -> (i64, i64, i64)  // Returns (gcd, x, y) where ax + by = gcd
```

### Modular Arithmetic

```rust
pub fn mod_pow(base: u64, exp: u64, modulus: u64) -> u64  // (base^exp) % modulus
pub fn mod_inv(a: i64, m: i64) -> Option<i64>  // Modular inverse using extended GCD
```

### Prime Utilities

```rust
pub fn is_prime(n: u64) -> bool  // Miller-Rabin for large numbers
pub fn primes_up_to(limit: usize) -> Vec<usize>  // Sieve of Eratosthenes
pub fn prime_factors(n: u64) -> Vec<u64>  // Prime factorization
```

### Combinations & Permutations

```rust
pub fn factorial(n: u64) -> u64
pub fn binomial(n: u64, k: u64) -> u64  // n choose k, with overflow checking
```

### Generic Constraints

- Uses `num-traits::Integer` for gcd/lcm (works with i32, i64, u32, u64, etc.)
- Specialized implementations for u64/i64 where needed (modular arithmetic, primes)

### Edge Cases

- `gcd(0, n) = n`
- `lcm` with overflow detection
- `mod_inv` returns None when inverse doesn't exist
- `factorial`/`binomial` panic on overflow (fail fast in competitive programming)

## Module 3: Pathfinding

### API Design Philosophy

Generic over node types using closures for neighbor generation. No need to implement traits - just provide a function that generates neighbors.

### BFS (Breadth-First Search)

```rust
pub fn bfs<N, FN, IN>(
    start: N,
    mut neighbors: FN,
    goal: impl Fn(&N) -> bool,
) -> Option<Vec<N>>
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
```

Returns shortest path from start to goal (or None if unreachable).

### BFS Variants

```rust
pub fn bfs_all_reachable<N, FN, IN>(start: N, neighbors: FN) -> HashMap<N, usize>
// Returns all reachable nodes with their distances from start

pub fn bfs_multiple_goals<N, FN, IN>(start: N, neighbors: FN, goals: &[N]) -> HashMap<N, Vec<N>>
// Find paths to multiple goals in one pass
```

### Dijkstra's Algorithm

```rust
pub fn dijkstra<N, FN, IN>(
    start: N,
    mut neighbors: FN,
    goal: impl Fn(&N) -> bool,
) -> Option<(Vec<N>, usize)>
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, usize)>,  // (neighbor, cost)
```

Returns path and total cost. Uses binary heap for efficiency.

### A* Search

```rust
pub fn astar<N, FN, IN, FH>(
    start: N,
    neighbors: FN,
    heuristic: FH,
    goal: impl Fn(&N) -> bool,
) -> Option<(Vec<N>, usize)>
where
    FH: Fn(&N) -> usize,  // Heuristic function
```

### DFS Utility

```rust
pub fn dfs_visit_all<N, FN, IN>(start: N, neighbors: FN) -> HashSet<N>
// Visit all reachable nodes via DFS (useful for connected components)
```

### Example Usage

```rust
// Grid pathfinding
let path = bfs(
    (0, 0),
    |&(x, y)| grid.neighbors4(x, y).map(|(nx, ny, _)| (nx, ny)),
    |&pos| pos == goal
);

// Graph with weighted edges
let (path, cost) = dijkstra(
    start_node,
    |&n| graph.get_neighbors(n),
    |&n| n == end_node
)?;
```

## Module 4: Parsing

### Number Extraction

```rust
pub fn extract_numbers(s: &str) -> Vec<i64>
// Extracts all numbers (including negatives) from a string
// Example: "move x=10, y=-5" -> [10, -5]

pub fn extract_ints<T: FromStr>(s: &str) -> Vec<T>
// Generic version for any numeric type
// Handles edge cases like overlapping signs

pub fn extract_unsigned(s: &str) -> Vec<u64>
// Only positive numbers, faster than generic version
```

### Pattern Matching

```rust
pub fn scan_pattern(s: &str, pattern: &str) -> Option<Vec<String>>
// Simple scanf-like matching using {} as wildcards
// Example: scan_pattern("move 10 steps", "move {} steps") -> Some(vec!["10"])
// Example: scan_pattern("x=5, y=10", "x={}, y={}") -> Some(vec!["5", "10"])
```

### Grid Parsing Helpers

```rust
pub fn parse_char_grid(lines: &[String]) -> Grid<char>
// Wrapper around Grid::parse_chars for convenience

pub fn parse_digit_grid(lines: &[String]) -> Grid<u8>
// Parse grids of single digits into numbers
```

### Line Splitting

```rust
pub fn split_groups(lines: &[String]) -> Vec<Vec<String>>
// Split on empty lines (common AoC pattern)
// Keeps non-empty lines in groups

pub fn split_on<F>(lines: &[String], predicate: F) -> Vec<Vec<String>>
where F: Fn(&str) -> bool
// Generic version - split on custom condition
```

### Utilities

```rust
pub fn parse_or_panic<T: FromStr>(s: &str) -> T
// Parse or panic with helpful message (for competitive programming speed)

pub fn must_match(s: &str, regex: &str) -> Vec<String>
// Regex matching that panics if no match (fail-fast)
```

### Implementation Notes

- Uses `regex` crate internally but exposes simple APIs
- All panicking functions clearly named (`or_panic`, `must_match`) for transparency
- Optimized for competitive programming: prefer panic over Result when input should always be valid

## Testing Strategy

### Test Organization

Each module gets its own test module:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    // Unit tests here
}
```

### Coverage Requirements

For each function:
- **Happy path** - Normal usage with typical inputs
- **Edge cases** - Empty inputs, boundary values, zeros, negatives
- **Error cases** - Invalid inputs where applicable
- **AoC patterns** - Real-world examples from past puzzles

### Example Test Structure

```rust
#[test]
fn test_gcd_basic() {
    assert_eq!(gcd(48, 18), 6);
}

#[test]
fn test_gcd_edge_cases() {
    assert_eq!(gcd(0, 5), 5);
    assert_eq!(gcd(5, 0), 5);
    assert_eq!(gcd(1, 1), 1);
}

#[test]
fn test_gcd_properties() {
    // gcd(a, b) * lcm(a, b) = a * b
    assert_eq!(gcd(12, 8) * lcm(12, 8), 12 * 8);
}
```

### TDD Workflow

1. Write failing test for new function
2. Implement minimal code to pass test
3. Add edge case tests
4. Refactor implementation
5. Repeat for next function

## Implementation Order

1. **Math module** - Standalone, no dependencies, easiest to test
2. **Parsing module** - Standalone, useful for testing other modules
3. **Grid module** - Uses parsing for construction
4. **Pathfinding module** - Can use Grid for integration tests

## Success Criteria

- All functions have unit tests with 100% coverage
- Edge cases documented and tested
- Integration tests using real AoC puzzle patterns
- Documentation with examples for each public function
- Benchmarks for performance-critical functions (pathfinding, primes)

## Non-Goals

- 3D grids (can be added later if needed)
- Hex grids (specialized, not worth the complexity)
- Advanced graph structures (overkill for AoC)
- Arbitrary precision arithmetic (AoC rarely needs it)

## Future Considerations

If commonly needed in future AoC years:
- Union-find / Disjoint set data structure
- Range utilities (overlaps, merging)
- Interval trees
- More specialized iterators
