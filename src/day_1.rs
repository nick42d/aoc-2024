fn parse_test_data(data: String) -> (Vec<usize>, Vec<usize>) {
    let mut list_1 = vec![];
    let mut list_2 = vec![];
    for line in TEST_DATA.lines() {
        let (one, two) = line.split_once("   ").unwrap();
        list_1.push(str::parse(one).unwrap());
        list_2.push(str::parse(two).unwrap());
    }
    (list_1, list_2)
}
fn compare_lists(mut list_1: Vec<usize>, mut list_2: Vec<usize>) -> usize {
    list_1.sort();
    list_2.sort();
    list_1
        .into_iter()
        .zip(list_2.into_iter())
        .fold(0, |acc, (e1, e2)| acc + e1.abs_diff(e2))
}
fn compare_list_similarity(mut list_1: Vec<usize>, list_2: Vec<usize>) -> usize {
    list_1.sort();
    list_1
        .into_iter()
        .map(|e1| e1 * list_2.iter().filter(|e2| **e2 == e1).count())
        .reduce(|acc, e| acc + e)
        .unwrap()
}

pub fn part_1(file: String) {
    let (l1, l2) = parse_test_data(file);
    println!("{}", compare_lists(l1, l2));
}
pub fn part_2(file: String) {
    let (l1, l2) = parse_test_data(file);
    println!("{}", compare_list_similarity(l1, l2));
}
