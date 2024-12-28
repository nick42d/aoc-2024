use std::collections::{BTreeSet, HashMap, HashSet};

use crate::utils::Bfs;

fn parse_input(s: &str) -> impl Iterator<Item = (&str, &str)> {
    s.lines().map(|line| line.split_once("-").unwrap())
}

fn input_to_graph(s: &str) -> HashMap<String, BTreeSet<String>> {
    let mut out = HashMap::new();
    for (v1, v2) in parse_input(s) {
        out.entry(v1.to_string())
            .and_modify(|v: &mut BTreeSet<_>| {
                v.insert(v2.to_string());
            })
            .or_insert([v2.to_string()].into());
        out.entry(v2.to_string())
            .and_modify(|v: &mut BTreeSet<_>| {
                v.insert(v1.to_string());
            })
            .or_insert([v1.to_string()].into());
    }
    out
}

#[derive(Eq, PartialEq, Debug, Hash, Clone, PartialOrd, Ord)]
enum SearchState {
    Keys(Vec<String>),
    Init,
}

fn get_sets_len(s: &str, len: usize) -> HashSet<Vec<String>> {
    let graph = input_to_graph(s);
    let bfs = Bfs::new_with_refdata(
        SearchState::Init,
        |_, _| false,
        |_| None,
        |state, g| match state {
            SearchState::Keys(v) => Box::new(
                v.clone()
                    .into_iter()
                    .flat_map(|k| g.get(&k).cloned().into_iter())
                    .reduce(|mut acc, e| acc.intersection(&e).cloned().collect())
                    .map(move |set| {
                        set.into_iter().map(move |x| {
                            let mut vc = v.clone();
                            vc.push(x.to_owned());
                            (SearchState::Keys(vc), ())
                        })
                    })
                    .into_iter()
                    .flatten(),
            ) as Box<dyn Iterator<Item = (SearchState, ())>>,
            SearchState::Init => Box::new(
                g.keys()
                    .map(|k| (SearchState::Keys([k.to_string()].into()), ())),
            ) as Box<dyn Iterator<Item = (SearchState, ())>>,
        },
        &graph,
    )
    .with_max_len(len)
    .execute();
    bfs.into_iter()
        .filter_map(|(k, v)| match k {
            SearchState::Keys(vec) => {
                let mut set: Vec<_> = vec.into_iter().collect();
                set.sort();
                set.dedup();
                if set.len() == len {
                    Some(set)
                } else {
                    None
                }
            }
            SearchState::Init => None,
        })
        .collect()
}

fn get_largest_set(s: &str) -> Vec<String> {
    let graph = input_to_graph(s);
    let bfs = Bfs::new_with_refdata(
        SearchState::Init,
        |_, _| false,
        |_| None,
        |state, g| match state {
            SearchState::Keys(v) => Box::new(
                v.clone()
                    .into_iter()
                    .flat_map(|k| g.get(&k).cloned().into_iter())
                    .reduce(|mut acc, e| acc.intersection(&e).cloned().collect())
                    .map(move |set| {
                        set.into_iter().map(move |x| {
                            let mut vc = v.clone();
                            vc.push(x.to_owned());
                            (SearchState::Keys(vc), ())
                        })
                    })
                    .into_iter()
                    .flatten(),
            ) as Box<dyn Iterator<Item = (SearchState, ())>>,
            SearchState::Init => Box::new(
                g.keys()
                    .map(|k| (SearchState::Keys([k.to_string()].into()), ())),
            ) as Box<dyn Iterator<Item = (SearchState, ())>>,
        },
        &graph,
    )
    .in_debug_mode()
    .execute();
    let (largest_set, _) = bfs
        .into_iter()
        .max_by_key(|(k, v)| match k {
            SearchState::Keys(vec) => {
                let mut set: Vec<_> = vec.into_iter().collect();
                set.sort();
                set.dedup();
                set.len()
            }
            SearchState::Init => 0,
        })
        .unwrap();
    match largest_set {
        SearchState::Keys(vec) => vec,
        SearchState::Init => unreachable!(),
    }
}

pub(crate) fn part_1(input: String) {
    let sets = get_sets_len(&input, 3)
        .into_iter()
        .filter(|v| v.iter().any(|s| s.starts_with('t')))
        .collect::<Vec<_>>();
    println!("Matching computers: {}", sets.len());
}

pub(crate) fn part_2(input: String) {
    let set = get_largest_set(&input);
    println!("Matching set: {:?}", set);
}

const TEST_DATA: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

#[test]
fn test_part_1_all_sets() {
    let sets = get_sets_len(TEST_DATA, 3);
    assert_eq!(sets.len(), 12)
}

#[test]
fn test_part_1() {
    let sets = get_sets_len(TEST_DATA, 3)
        .into_iter()
        .filter(|v| v.iter().any(|s| s.starts_with('t')))
        .collect::<Vec<_>>();
    assert_eq!(sets.len(), 7)
}

#[test]
fn test_part_2() {
    let set = get_largest_set(TEST_DATA);
    assert_eq!(
        set.into_iter().collect::<BTreeSet<_>>(),
        BTreeSet::from_iter(["co", "de", "ka", "ta"].into_iter().map(|s| s.to_string()))
    )
}
