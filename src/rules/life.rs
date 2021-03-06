use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use super::from_stream::*;
use super::BoxRule;
// Cell
// 一般生命游戏元胞类型
#[derive(PartialEq, Eq, Clone)]
pub enum Cell {
	Live,
	Dead
}
use self::Cell::*;
impl Debug for Cell {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		match self {
			Live => write!(f, "#"),
			Dead => write!(f, "."),
		}
	}
}
impl Display for Cell {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		match self {
			Live => write!(f, "\u{1b}[7m -\u{1b}[0m"),
			Dead => write!(f, " -"),
		}
	}
}
impl FromStream for Cell {
	fn from_stream(s: &str) -> Option<(usize, Self)> {
		match s.chars().next() {
			Some('#') => Some((1, Live)),
			Some('.') => Some((1, Dead)),
			_ => None,
		}
	}
}

#[allow(dead_code)]
pub fn rule(birth: &[usize], save: &[usize]) -> BoxRule<Cell> {
	let birth = birth.to_owned();
	let save = save.to_owned();
	Box::new(move |nw: &Cell, n: &Cell, ne: &Cell,
	                w: &Cell, c: &Cell,  e: &Cell,
	               sw: &Cell, s: &Cell, se: &Cell| -> Cell {
		let [out_sum] = count!{Live in nw,n,ne,w,e,sw,s,se};
		match c {
			Live => if save.contains(&out_sum) {Live} else {Dead},
			Dead => if birth.contains(&out_sum) {Live} else {Dead},
		}
	})
}
#[allow(dead_code)]
pub fn rule_h(birth: &[usize], save: &[usize]) -> BoxRule<Cell> {
	let birth = birth.to_owned();
	let save = save.to_owned();
	Box::new(move | nw: &Cell, n: &Cell, _ne: &Cell,
	                 w: &Cell, c: &Cell,   e: &Cell,
	               _sw: &Cell, s: &Cell,  se: &Cell| -> Cell {
		let [out_sum] = count!{Live in nw,n,w,e,s,se};
		match c {
			Live => if save.contains(&out_sum) {Live} else {Dead},
			Dead => if birth.contains(&out_sum) {Live} else {Dead},
		}
	})
}
#[allow(unreachable_patterns, dead_code)]
pub fn non_totalistic_rule(birth: &str, save: &str) -> BoxRule<Cell> {
	let birth = non_totalistic_closure!(Cell; Live, birth);
	let save = non_totalistic_closure!(Cell; Live, save);
	Box::new(move |nw: &Cell, n: &Cell, ne: &Cell,
	                w: &Cell, c: &Cell,  e: &Cell,
	               sw: &Cell, s: &Cell, se: &Cell| -> Cell {
		match c {
			Live => if save(nw,n,ne,w,e,sw,s,se) {Live} else {Dead},
			Dead => if birth(nw,n,ne,w,e,sw,s,se) {Live} else {Dead},
		}
	})
}
#[allow(unreachable_patterns, dead_code)]
pub fn non_totalistic_rule_h(birth: &str, save: &str) -> BoxRule<Cell> {
	let birth = non_totalistic_closure_h!(Cell; Live, birth);
	let save = non_totalistic_closure_h!(Cell; Live, save);
	Box::new(move | nw: &Cell, n: &Cell, _ne: &Cell,
	                 w: &Cell, c: &Cell,   e: &Cell,
	               _sw: &Cell, s: &Cell,  se: &Cell| -> Cell {
		match c {
			Live => if save(nw,w,n,s,e,se) {Live} else {Dead},
			Dead => if birth(nw,w,n,s,e,se) {Live} else {Dead},
		}
	})
}