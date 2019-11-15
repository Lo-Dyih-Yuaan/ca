use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use super::FromStream;
use super::BoxRule;
// Cell
// 一般生命游戏元胞类型
#[derive(PartialEq, Eq, Clone)]
pub enum Cell {
	Live,
	Dead
}
impl Debug for Cell {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		match self {
			Cell::Live => write!(f, "#"),
			Cell::Dead => write!(f, "."),
		}
	}
}
impl Display for Cell {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		match self {
			Cell::Live => write!(f, "\u{1b}[7m -\u{1b}[0m"),
			Cell::Dead => write!(f, " -"),
		}
	}
}
impl FromStream for Cell {
	fn from_stream(s: &str) -> (usize, Option<Self>) {
		match s.chars().next() {
			Some('#') => (1, Some(Cell::Live)),
			Some('.') => (1, Some(Cell::Dead)),
			_ => (0, None),
		}
	}
}

#[allow(dead_code)]
pub fn rule(birth: &'static[usize], save: &'static[usize]) -> BoxRule<Cell> {
	Box::new(move |nw: &Cell, n: &Cell, ne: &Cell,
	                w: &Cell, c: &Cell,  e: &Cell,
	               sw: &Cell, s: &Cell, se: &Cell| -> Cell {
		let mut out_sum: usize = 0;
		count!{$
			Cell::Live => out_sum;
			*nw,*n,*ne,*w,*e,*sw,*s,*se
		};
		match c {
			Cell::Live =>
				if save.contains(&out_sum) {Cell::Live} else {Cell::Dead},
			Cell::Dead =>
				if birth.contains(&out_sum) {Cell::Live} else {Cell::Dead},
		}
	})
}
#[allow(unreachable_patterns, dead_code)]
pub fn non_totalistic_rule(birth: &'static str, save: &'static str) -> BoxRule<Cell> {
	let b_fun = non_totalistic_closure!(Cell; Cell::Live, birth);
	let s_fun = non_totalistic_closure!(Cell; Cell::Live, save);
	Box::new(move |nw: &Cell, n: &Cell, ne: &Cell,
	                w: &Cell, c: &Cell,  e: &Cell,
	               sw: &Cell, s: &Cell, se: &Cell| -> Cell {
		match c {
			Cell::Live =>
				if s_fun(nw,n,ne,w,e,sw,s,se) {Cell::Live} else {Cell::Dead},
			Cell::Dead =>
				if b_fun(nw,n,ne,w,e,sw,s,se) {Cell::Live} else {Cell::Dead},
		}
	})
}