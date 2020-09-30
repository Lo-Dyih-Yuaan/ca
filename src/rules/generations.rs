use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use super::from_stream::*;
use super::tools::u32_print_str;
use super::BoxRule;
// GenerationsCell
// 非立即死亡的生命游戏元胞类型
#[derive(PartialEq, Eq, Clone)]
pub enum Cell {
	Live,
	Dead,
	Generations(usize)
}
use self::Cell::*;
impl Debug for Cell {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		match self {
			Live => write!(f, "#"),
			Dead => write!(f, "."),
			Generations(n) =>
				write!(f, "{}", u32_print_str(*n as u32))
		}
	}
}
impl Display for Cell {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		match self {
			Live => write!(f, "\u{1b}[7m -\u{1b}[0m"),
			Dead => write!(f, " -"),
			Generations(n) =>
				write!(f, "\u{1b}[48;5;8m{:>2}\u{1b}[0m", u32_print_str(*n as u32)),
		}
	}
}
impl FromStream for Cell {
	fn from_stream(s: &str) -> Option<(usize, Self)> {
		match s.chars().next() {
			Some('#') => Some((1, Live)),
			Some('.') => Some((1, Dead)),
			_ => match u32::from_stream(s) {
				Some((len, n)) => Some((len, Generations(n as usize))),
				None => None,
			}
		}
	}
}

#[allow(dead_code)]
pub fn rule(number: usize, birth: &[usize], save: &[usize]) -> BoxRule<Cell> {
	let birth = birth.to_owned();
	let save = save.to_owned();
	if number == 0 {
		unreachable!();
	}
	Box::new(move |nw: &Cell, n: &Cell, ne: &Cell,
	                w: &Cell, c: &Cell,  e: &Cell,
	               sw: &Cell, s: &Cell, se: &Cell| -> Cell {
		let [out_sum] = count!{Live in nw,n,ne,w,e,sw,s,se};
		match c {
			Live => if save.contains(&out_sum) {Live} else {Generations(0)},
			Dead => if birth.contains(&out_sum) {Live} else {Dead},
			Generations(n) =>
				if n+1 < number {Generations(n+1)}
				else if n+1 == number {Dead}
				else {unreachable!()}
		}
	})
}

#[allow(dead_code)]
pub fn rule_h(number: usize, birth: &[usize], save: &[usize]) -> BoxRule<Cell> {
	let birth = birth.to_owned();
	let save = save.to_owned();
	if number == 0 {
		unreachable!();
	}
	Box::new(move | nw: &Cell, n: &Cell, _ne: &Cell,
	                 w: &Cell, c: &Cell,   e: &Cell,
	               _sw: &Cell, s: &Cell,  se: &Cell| -> Cell {
		let [out_sum] = count!{Live in nw,n,w,e,s,se};
		match c {
			Live => if save.contains(&out_sum) {Live} else {Generations(0)},
			Dead => if birth.contains(&out_sum) {Live} else {Dead},
			Generations(n) =>
				if n+1 < number {Generations(n+1)}
				else if n+1 == number {Dead}
				else {unreachable!()}
		}
	})
}

#[allow(unreachable_patterns, dead_code)]
pub fn non_totalistic_rule(number: usize, birth: &str, save: &str) -> BoxRule<Cell> {
	if number == 0 {
		unreachable!();
	}
	let birth = non_totalistic_closure!(Cell; Live, birth);
	let save = non_totalistic_closure!(Cell; Live, save);
	Box::new(move |nw: &Cell, n: &Cell, ne: &Cell,
	                w: &Cell, c: &Cell,  e: &Cell,
	               sw: &Cell, s: &Cell, se: &Cell| -> Cell {
		match c {
			Live => if save(nw,n,ne,w,e,sw,s,se) {Live} else {Generations(0)},
			Dead => if birth(nw,n,ne,w,e,sw,s,se) {Live} else {Dead},
			Generations(n) =>
				if n+1 < number {Generations(n+1)}
				else if n+1 == number {Dead}
				else {unreachable!()}
		}
	})
}

#[allow(unreachable_patterns, dead_code)]
pub fn non_totalistic_rule_h(number: usize, birth: &str, save: &str) -> BoxRule<Cell> {
	if number == 0 {
		unreachable!();
	}
	let birth = non_totalistic_closure_h!(Cell; Live, birth);
	let save = non_totalistic_closure_h!(Cell; Live, save);
	Box::new(move | nw: &Cell, n: &Cell, _ne: &Cell,
	                 w: &Cell, c: &Cell,   e: &Cell,
	               _sw: &Cell, s: &Cell,  se: &Cell| -> Cell {
		match c {
			Live => if save(nw,w,n,s,e,se) {Live} else {Generations(0)},
			Dead => if birth(nw,w,n,s,e,se) {Live} else {Dead},
			Generations(n) =>
				if n+1 < number {Generations(n+1)}
				else if n+1 == number {Dead}
				else {unreachable!()}
		}
	})
}