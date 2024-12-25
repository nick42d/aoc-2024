use std::{
    borrow::Borrow,
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::{Debug, Display},
    hash::Hash,
    marker::PhantomData,
    ops::Add,
};

pub trait Tracking<M: Clone> {
    type Output: Default + Clone;
    fn push(o1: &Self::Output, m: M) -> Self::Output;
    fn len(o: &Self::Output) -> usize;
}

pub struct WithHistory;
impl<M: Clone> Tracking<M> for WithHistory {
    type Output = Vec<M>;
    fn push(o1: &Self::Output, m: M) -> Self::Output {
        let mut o1 = o1.clone();
        o1.push(m);
        o1
    }
    fn len(o: &Self::Output) -> usize {
        o.len()
    }
}
pub struct WithDistance;
impl<M: Clone> Tracking<M> for WithDistance {
    type Output = usize;
    fn push(o1: &Self::Output, m: M) -> Self::Output {
        o1 + 1
    }
    fn len(o: &Self::Output) -> usize {
        *o
    }
}

pub struct Bfs<'a, T, FG, FN, R, M, Tr> {
    init: T,
    goal_check: FG,
    get_neighbours: FN,
    reference_data: &'a R,
    message_type: PhantomData<M>,
    debug: bool,
    tracking: Tr,
    max_len: Option<usize>,
}
impl<T, FG, FN, R, M> Bfs<'_, T, FG, FN, R, M, WithDistance> {
    pub fn new<'b, I>(
        init: T,
        goal_check: impl Fn(&T) -> bool,
        get_neighbours: impl Fn(T) -> I,
    ) -> Bfs<
        'static,
        T,
        impl Fn(&'b T, &'static R) -> bool,
        impl Fn(T, &'static R) -> I,
        (),
        M,
        WithDistance,
    >
    where
        I: IntoIterator<Item = (T, M)>,
    {
        let goal_check = move |t, _| goal_check(t);
        let get_neighbours = move |t, _| get_neighbours(t);
        Bfs {
            init,
            goal_check,
            get_neighbours,
            reference_data: &(),
            debug: false,
            message_type: PhantomData,
            tracking: WithDistance,
            max_len: None,
        }
    }
    pub fn new_with_refdata<'a, I>(
        init: T,
        goal_check: FG,
        get_neighbours: FN,
        reference_data: &'a R,
    ) -> Bfs<'a, T, FG, FN, R, M, WithDistance>
    where
        I: IntoIterator<Item = (T, M)> + 'a,
        FG: Fn(&T, &R) -> bool,
        FN: Fn(T, &'a R) -> I,
        T: Eq + Hash + Clone + Debug,
        M: Clone,
    {
        Bfs {
            init,
            goal_check,
            get_neighbours,
            reference_data,
            debug: false,
            message_type: PhantomData,
            tracking: WithDistance,
            max_len: None,
        }
    }
}
impl<T, FG, FN, R, M, Tr> Bfs<'_, T, FG, FN, R, M, Tr> {
    pub fn with_history<'a>(mut self) -> Bfs<'a, T, FG, FN, R, M, WithHistory>
    where
        Self: 'a,
    {
        let Self {
            init,
            goal_check,
            get_neighbours,
            reference_data,
            message_type,
            debug,
            tracking,
            max_len,
        } = self;
        Bfs {
            init,
            goal_check,
            get_neighbours,
            reference_data,
            message_type,
            debug,
            tracking: WithHistory,
            max_len,
        }
    }
    pub fn with_max_len(mut self, max_len: usize) -> Self {
        self.max_len = Some(max_len);
        self
    }
    pub fn in_debug_mode(mut self) -> Self {
        self.debug = true;
        self
    }
}
impl<'a, T, FG, FN, R, M, Tr> Bfs<'a, T, FG, FN, R, M, Tr> {
    /// Returns list of all visited locations and the moves taken.
    pub fn execute<I>(self) -> HashMap<T, Tr::Output>
    where
        FG: Fn(&T, &R) -> bool,
        FN: Fn(T, &'a R) -> I,
        I: IntoIterator<Item = (T, M)> + 'a,
        T: Eq + Hash + Clone + Debug,
        M: Clone,
        Tr: Tracking<M>,
    {
        let Self {
            init,
            goal_check,
            get_neighbours,
            reference_data,
            message_type,
            debug,
            tracking,
            max_len,
        } = self;
        let mut queue = VecDeque::new();
        let mut explored = HashMap::new();
        explored.insert(init.clone(), Tr::Output::default());
        queue.push_front((init, Tr::Output::default()));
        if debug {
            println!("Running BFS in debug mode");
        }
        while let Some((next_to_visit, tracking)) = queue.pop_back() {
            if debug {
                println!("Reached len {}", Tr::len(&tracking));
            }
            if goal_check(&next_to_visit, reference_data) {
                break;
            }
            if let Some(max_len) = max_len {
                if Tr::len(&tracking) > max_len {
                    break;
                }
            }
            for (neighbour, m) in get_neighbours(next_to_visit, reference_data) {
                if !explored.contains_key(&neighbour) {
                    let next_tracking = Tr::push(&tracking, m);
                    explored.insert(neighbour.clone(), next_tracking.clone());
                    queue.push_front((neighbour, next_tracking));
                }
            }
        }
        explored
    }
}
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
