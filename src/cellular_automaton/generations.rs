use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use super::u32_print_str;
use super::FromStream;
use super::BoxRule;
// GenerationsCell
// 非立即死亡的生命游戏元胞类型
#[derive(PartialEq, Eq, Clone)]
pub enum Cell {
	Live,
	Dead,
	Generations(u32)
}
impl Debug for Cell {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		match self {
			Cell::Live => write!(f, "#"),
			Cell::Dead => write!(f, "."),
			Cell::Generations(n) =>
				write!(f, "{}", u32_print_str(*n))
		}
	}
}
impl Display for Cell {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		match self {
			Cell::Live => write!(f, "\u{1b}[7m -\u{1b}[0m"),
			Cell::Dead => write!(f, " -"),
			Cell::Generations(n @ 0 ... 25) =>
				write!(f, "\u{1b}[48;5;8m {}\u{1b}[0m", u32_print_str(*n)),
			Cell::Generations(n) =>
				write!(f, "\u{1b}[48;5;8m{}\u{1b}[0m", u32_print_str(*n))
		}
	}
}
impl FromStream for Cell {
	fn from_stream(s: &str) -> (usize, Option<Self>) {
		match s.chars().next() {
			Some('#') => (1, Some(Cell::Live)),
			Some('.') => (1, Some(Cell::Dead)),
			_ => match u32::from_stream(s) {
				(len, None) => (len, None),
				(len, Some(n)) => (len, Some(Cell::Generations(n)))
			}
		}
	}
}

#[allow(dead_code)]
pub fn rule(number: u32, birth: &'static[usize], save: &'static[usize]) -> BoxRule<Cell> {
	if number == 0 {
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
				_ => unreachable!()
			}
		})
	} else {
		Box::new(move |nw: &Cell, n: &Cell, ne: &Cell,
		                w: &Cell, c: &Cell,  e: &Cell,
		               sw: &Cell, s: &Cell, se: &Cell| -> Cell {
			if let Cell::Generations(n) = c {
				return
					if n+1 < number {Cell::Generations(n+1)}
					else if n+1 == number {Cell::Dead}
					else {unreachable!()};
			}
			let mut out_sum: usize = 0;
			count!{$
				Cell::Live => out_sum;
				*nw,*n,*ne,*w,*e,*sw,*s,*se
			};
			match c {
				Cell::Live =>
					if save.contains(&out_sum) {Cell::Live} else {Cell::Generations(0)},
				Cell::Dead =>
					if birth.contains(&out_sum) {Cell::Live} else {Cell::Dead},
				_ => unreachable!()
			}
		})
	}
}
#[allow(unreachable_patterns, dead_code)]
pub fn non_totalistic_rule(number: u32, birth: &'static str, save: &'static str) -> BoxRule<Cell> {
	let b_fun = non_totalistic_closure!(Cell; Cell::Live, birth);
	let s_fun = non_totalistic_closure!(Cell; Cell::Live, save);
	if number == 0 {
		Box::new(move |nw: &Cell, n: &Cell, ne: &Cell,
		                w: &Cell, c: &Cell,  e: &Cell,
		               sw: &Cell, s: &Cell, se: &Cell| -> Cell {
			match c {
				Cell::Live =>
					if s_fun(nw,n,ne,w,e,sw,s,se) {Cell::Live} else {Cell::Dead},
				Cell::Dead =>
					if b_fun(nw,n,ne,w,e,sw,s,se) {Cell::Live} else {Cell::Dead},
				_ => unreachable!()
			}
		})
	} else {
		Box::new(move |nw: &Cell, n: &Cell, ne: &Cell,
		                w: &Cell, c: &Cell,  e: &Cell,
		               sw: &Cell, s: &Cell, se: &Cell| -> Cell {
			if let Cell::Generations(n) = c {
				return
					if n+1 < number {Cell::Generations(n+1)}
					else if n+1 == number {Cell::Dead}
					else {unreachable!()};
			}
			match c {
				Cell::Live =>
					if s_fun(nw,n,ne,w,e,sw,s,se) {Cell::Live} else {Cell::Generations(0)},
				Cell::Dead =>
					if b_fun(nw,n,ne,w,e,sw,s,se) {Cell::Live} else {Cell::Dead},
				_ => unreachable!()
			}
		})
	}
}