#[macro_use]
extern crate ca;
use ca::rules;
use ca::pattern::Pattern;
use ca::golly;
use std::convert::TryFrom;

#[test]
fn test_ntaa_1_1() {
	let tree = "num_states=5
num_neighbors=4
num_nodes=43
1 0 1 4 4 1
1 0 1 2 3 1
1 0 2 4 4 4
1 0 3 4 4 4
2 0 1 2 3 0
1 0 2 2 3 4
1 0 3 2 3 4
2 1 1 5 6 1
1 0 1 4 4 4
2 2 5 2 3 8
2 3 6 3 2 8
2 0 1 8 8 0
3 4 7 9 10 11
1 0 1 2 3 4
2 5 13 5 6 13
2 6 13 6 5 13
2 1 1 13 13 1
3 7 7 14 15 16
2 8 13 8 8 8
3 9 14 9 10 18
2 2 5 2 2 8
3 10 15 10 20 18
3 11 16 18 18 11
4 12 17 19 21 22
2 5 5 5 6 13
2 6 6 6 5 13
3 7 16 24 25 16
2 13 13 13 13 13
3 16 16 27 27 16
3 24 27 14 15 27
2 5 13 5 5 13
3 25 27 15 30 27
4 26 28 29 31 28
2 5 13 13 13 13
3 14 33 14 15 27
3 18 27 18 18 18
4 19 34 19 21 35
2 6 13 13 13 13
3 15 37 15 30 27
3 20 30 20 20 18
4 21 38 21 39 35
4 22 28 35 35 22
5 23 32 36 40 41
";
	let (rf, _, _) = rule!{NoTimeAtAll};
	let states = ".-01@";
	let states = &Pattern::try_from(format!("{}$", states)).unwrap().get_data()[0];
	let mut rt = golly::RuleTree::new(states.to_vec());
	rt.create_von_neumann_tree(&rf);
	assert_eq!(format!("{:?}", rt), tree);
}

#[test]
fn test_ntaa_01_03() {
	let tree = "num_states=5
num_neighbors=4
num_nodes=50
1 0 1 4 4 1
1 0 1 2 3 1
1 0 2 4 4 4
1 0 3 4 4 4
2 0 1 2 3 0
1 0 2 2 3 4
1 0 3 2 3 4
2 1 1 5 6 1
1 0 1 4 4 4
2 2 5 3 3 8
2 3 6 3 2 8
2 0 1 8 8 0
3 4 7 9 10 11
1 0 1 2 3 4
2 5 13 6 6 13
2 6 13 6 5 13
2 1 1 13 13 1
3 7 7 14 15 16
2 3 6 2 2 8
2 8 13 8 8 8
3 9 14 10 18 19
2 2 5 2 3 8
3 10 15 18 21 19
3 11 16 19 19 11
4 12 17 20 22 23
2 5 5 6 6 13
2 6 6 6 5 13
3 7 16 25 26 16
2 13 13 13 13 13
3 16 16 28 28 16
2 6 13 5 5 13
3 25 28 15 30 28
2 5 13 5 6 13
3 26 28 30 32 28
4 27 29 31 33 29
2 5 13 13 13 13
3 14 35 15 30 28
2 3 6 2 3 8
2 2 5 3 2 8
3 10 15 37 38 19
2 2 5 2 2 8
3 18 30 38 40 19
3 19 28 19 19 19
4 20 36 39 41 42
2 6 13 13 13 13
3 15 44 30 32 28
3 21 32 40 18 19
4 22 45 41 46 42
4 23 29 42 42 23
5 24 34 43 47 48
";
	let (rf, _, _) = rule!{NoTimeAtAll - B 0 1 / T 0 3};
	let states = ".-01@";
	let states = &Pattern::try_from(format!("{}$", states)).unwrap().get_data()[0];
	let mut rt = golly::RuleTree::new(states.to_vec());
	rt.create_von_neumann_tree(&rf);
	assert_eq!(format!("{:?}", rt), tree);
}