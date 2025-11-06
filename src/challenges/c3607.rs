//! You are given an integer c representing c power stations, each with a unique identifier id from 1 to c (1‑based indexing).
//!
//! These stations are interconnected via n bidirectional cables, represented by a 2D array connections, where each element connections[i] = [ui, vi] indicates a connection between station ui and station vi. Stations that are directly or indirectly connected form a power grid.
//!
//! Initially, all stations are online (operational).
//!
//! You are also given a 2D array queries, where each query is one of the following two types:
//!
//! [1, x]: A maintenance check is requested for station x. If station x is online, it resolves the check by itself. If station x is offline, the check is resolved by the operational station with the smallest id in the same power grid as x. If no operational station exists in that grid, return -1.
//!
//! [2, x]: Station x goes offline (i.e., it becomes non-operational).
//!
//! Return an array of integers representing the results of each query of type [1, x] in the order they appear.
//!
//! Note: The power grid preserves its structure; an offline (non‑operational) node remains part of its grid and taking it offline does not alter connectivity.
//!
//!  
//!
//! Example 1:
//!
//! Input: c = 5, connections = [[1,2],[2,3],[3,4],[4,5]], queries = [[1,3],[2,1],[1,1],[2,2],[1,2]]
//!
//! Output: [3,2,3]
//!
//! Explanation:
//!
//!
//!
//! Initially, all stations {1, 2, 3, 4, 5} are online and form a single power grid.
//! Query [1,3]: Station 3 is online, so the maintenance check is resolved by station 3.
//! Query [2,1]: Station 1 goes offline. The remaining online stations are {2, 3, 4, 5}.
//! Query [1,1]: Station 1 is offline, so the check is resolved by the operational station with the smallest id among {2, 3, 4, 5}, which is station 2.
//! Query [2,2]: Station 2 goes offline. The remaining online stations are {3, 4, 5}.
//! Query [1,2]: Station 2 is offline, so the check is resolved by the operational station with the smallest id among {3, 4, 5}, which is station 3.
//! Example 2:
//!
//! Input: c = 3, connections = [], queries = [[1,1],[2,1],[1,1]]
//!
//! Output: [1,-1]
//!
//! Explanation:
//!
//! There are no connections, so each station is its own isolated grid.
//! Query [1,1]: Station 1 is online in its isolated grid, so the maintenance check is resolved by station 1.
//! Query [2,1]: Station 1 goes offline.
//! Query [1,1]: Station 1 is offline and there are no other stations in its grid, so the result is -1.
//!  
//!
//! Constraints:
//!
//! 1 <= c <= 105
//! 0 <= n == connections.length <= min(105, c * (c - 1) / 2)
//! connections[i].length == 2
//! 1 <= ui, vi <= c
//! ui != vi
//! 1 <= queries.length <= 2 * 105
//! queries[i].length == 2
//! queries[i][0] is either 1 or 2.
//! 1 <= queries[i][1] <= c

use super::*;

use std::collections::HashMap;

struct Dsu {
    parent: Vec<usize>,
}

impl Dsu {
    fn new(size: usize) -> Self {
        let mut parent = vec![0; size];
        for (i, parent) in parent.iter_mut().enumerate().take(size) {
            *parent = i;
        }
        Self { parent }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn join(&mut self, u: usize, v: usize) {
        let root_u = self.find(u);
        let root_v = self.find(v);
        if root_u != root_v {
            self.parent[root_v] = root_u;
        }
    }
}

impl Solution {
    pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let c = c as usize;
        let mut dsu = Dsu::new(c + 1);

        for p in connections {
            dsu.join(p[0] as usize, p[1] as usize);
        }

        let mut online = vec![true; c + 1];
        let mut offline_counts = vec![0; c + 1];
        let mut minimum_online_stations: HashMap<usize, i32> = HashMap::new();

        for q in &queries {
            let op = q[0];
            let x = q[1] as usize;
            if op == 2 {
                online[x] = false;
                offline_counts[x] += 1;
            }
        }

        for (i, online) in online.iter().enumerate().take(c + 1).skip(1) {
            let root = dsu.find(i);
            minimum_online_stations.entry(root).or_insert(-1);

            let station = *minimum_online_stations.get(&root).unwrap();
            if *online && (station == -1 || station > i as i32) {
                minimum_online_stations.insert(root, i as i32);
            }
        }

        let mut ans = Vec::new();

        for i in (0..queries.len()).rev() {
            let op = queries[i][0];
            let x = queries[i][1] as usize;
            let root = dsu.find(x);
            let station = *minimum_online_stations.get(&root).unwrap();

            if op == 1 {
                if online[x] {
                    ans.push(x as i32);
                } else {
                    ans.push(station);
                }
            }

            if op == 2 {
                if offline_counts[x] > 1 {
                    offline_counts[x] -= 1;
                } else {
                    online[x] = true;
                    if station == -1 || station > x as i32 {
                        minimum_online_stations.insert(root, x as i32);
                    }
                }
            }
        }

        ans.reverse();
        ans
    }
}
