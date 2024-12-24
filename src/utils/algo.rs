use std::{
    borrow::Borrow,
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::{Debug, Display},
    hash::Hash,
    ops::Add,
};

/// Returns list of all visited locations and the moves taken.
pub fn generic_bfs<T, I, M, R>(
    init: T,
    // Function will short circuit if goal is reached.
    goal_check: impl Fn(&T, &R) -> bool,
    // Don't bother gettting neighbours if we are here, already in invalid state.
    state_shortcircuit: impl Fn(&T, &R) -> bool,
    get_neighbours: impl Fn(T, &R) -> I,
    reference_data: &R,
) -> HashMap<T, Vec<M>>
where
    T: Eq + Hash + Clone + Debug,
    M: Clone,
    I: IntoIterator<Item = (T, M)>,
{
    let mut queue = VecDeque::new();
    let mut explored = HashMap::new();
    explored.insert(init.clone(), vec![]);
    queue.push_front((init, vec![]));
    while let Some((next_to_visit, mut history)) = queue.pop_back() {
        if goal_check(&next_to_visit, reference_data) {
            break;
        }
        for (neighbour, m) in get_neighbours(next_to_visit, reference_data) {
            if !explored.contains_key(&neighbour) && !state_shortcircuit(&neighbour, reference_data)
            {
                let mut new_history = history.clone();
                new_history.push(m);
                explored.insert(neighbour.clone(), new_history.clone());
                queue.push_front((neighbour, new_history));
            }
        }
    }
    explored
}

/// Returns list of all visited locations number of moves to each.
pub fn generic_bfs_nohistory<T, I, M, R>(
    init: T,
    // Function will short circuit if goal is reached.
    goal_check: impl Fn(&T, &R) -> bool,
    // Don't bother gettting neighbours if we are here, already in invalid state.
    state_shortcircuit: impl Fn(&T, &R) -> bool,
    get_neighbours: impl Fn(T, &R) -> I,
    reference_data: &R,
) -> HashMap<T, usize>
where
    T: Eq + Hash + Clone + Debug,
    M: Clone,
    I: IntoIterator<Item = (T, M)>,
{
    let mut queue = VecDeque::new();
    let mut explored = HashMap::new();
    explored.insert(init.clone(), 0);
    queue.push_front((init, 0));
    while let Some((next_to_visit, w)) = queue.pop_back() {
        if goal_check(&next_to_visit, reference_data) {
            break;
        }
        for (neighbour, n_m) in get_neighbours(next_to_visit, reference_data) {
            if !explored.contains_key(&neighbour) && !state_shortcircuit(&neighbour, reference_data)
            {
                explored.insert(neighbour.clone(), w + 1);
                queue.push_front((neighbour, w + 1));
            }
        }
    }
    explored
}

/// Returns list of all visited locations number of moves to each. May not
/// necessarily have the fastest route.
pub fn generic_dfs_nohistory<T, I, M, R>(
    init: T,
    // Function will short circuit if goal is reached.
    goal_check: impl Fn(&T, &R) -> bool,
    // Don't bother gettting neighbours if we are here, already in invalid state.
    state_shortcircuit: impl Fn(&T, &R) -> bool,
    get_neighbours: impl Fn(T, &R) -> I,
    reference_data: &R,
) -> HashMap<T, usize>
where
    T: Eq + Hash + Clone + Debug,
    M: Clone,
    I: IntoIterator<Item = (T, M)>,
{
    let mut queue = Vec::new();
    let mut explored = HashMap::new();
    explored.insert(init.clone(), 0);
    queue.push((init, 0));
    while let Some((next_to_visit, w)) = queue.pop() {
        if goal_check(&next_to_visit, reference_data) {
            break;
        }
        for (neighbour, n_m) in get_neighbours(next_to_visit, reference_data) {
            if !explored.contains_key(&neighbour) && !state_shortcircuit(&neighbour, reference_data)
            {
                explored.insert(neighbour.clone(), w + 1);
                queue.push((neighbour, w + 1));
            }
        }
    }
    explored
}

#[derive(Debug)]
struct DijkstraNode<T, W, M> {
    node: T,
    dist: W,
    hist: Vec<M>,
}
impl<T, W, M> DijkstraNode<T, W, M> {
    fn new(val: T, dist: W, hist: impl IntoIterator<Item = M>) -> Self {
        Self {
            node: val,
            dist,
            hist: hist.into_iter().collect(),
        }
    }
    fn equal_to_node(&self, other: &Self) -> bool
    where
        T: PartialEq,
    {
        self.node == other.node
    }
}
// Required to implement PartialOrd
impl<T, W, M> PartialEq for DijkstraNode<T, W, M>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
    }
}
// Required to implement Ord
impl<T, W, M> Eq for DijkstraNode<T, W, M> where T: Eq {}
impl<T, W, M> PartialOrd for DijkstraNode<T, W, M>
where
    W: PartialOrd,
    T: PartialEq,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.dist.partial_cmp(&other.dist)
    }
}
impl<T, W, M> Ord for DijkstraNode<T, W, M>
where
    W: Ord,
    T: Eq,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist.cmp(&other.dist)
    }
}

/// Returns list of all visited locations, weight to get there, and the moves
/// taken.
pub fn generic_dijkstra<T, I, M, W, R>(
    init: T,
    // Function will short circuit if goal is reached.
    goal_check: impl Fn(&T, &R) -> bool,
    // Don't bother gettting neighbours if we are here, already in invalid state.
    state_shortcircuit: impl Fn(&T, &R) -> bool,
    get_neighbours: impl Fn(T, &R) -> I,
    reference_data: &R,
) -> HashMap<T, (W, Vec<M>)>
where
    T: Eq + Hash + Clone + Debug,
    M: Clone,
    W: Ord + Add<W, Output = W> + Default + Copy + Debug,
    I: IntoIterator<Item = (T, M, W)>,
{
    let mut queue = BinaryHeap::new();
    let mut explored = HashMap::new();
    explored.insert(init.clone(), (W::default(), vec![]));
    queue.push(Reverse(DijkstraNode::new(init, W::default(), vec![])));
    while let Some(Reverse(DijkstraNode { node, dist, hist })) = queue.pop() {
        if goal_check(&node, reference_data) {
            break;
        }
        for (neighbour, m, w) in get_neighbours(node, reference_data) {
            if explored.get(&neighbour).is_none_or(|(e_w, _)| e_w <= &w)
                && !state_shortcircuit(&neighbour, reference_data)
            {
                let mut new_history = hist.clone();
                new_history.push(m);
                explored.insert(neighbour.clone(), (dist + w, new_history.clone()));
                queue.push(Reverse(DijkstraNode::new(neighbour, dist + w, new_history)));
            }
        }
    }
    explored
}
