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
use self::Cell::*;
impl Debug for Cell {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		match self {
			Live => write!(f, "#"),
			Dead => write!(f, "."),
			Generations(n) =>
				write!(f, "{}", u32_print_str(*n))
		}
	}
}
impl Display for Cell {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		match self {
			Live => write!(f, "\u{1b}[7m -\u{1b}[0m"),
			Dead => write!(f, " -"),
			Generations(n) =>
				write!(f, "\u{1b}[48;5;8m{:>2}\u{1b}[0m", u32_print_str(*n)),
		}
	}
}
impl FromStream for Cell {
	fn from_stream(s: &str) -> Option<(usize, Self)> {
		match s.chars().next() {
			Some('#') => Some((1, Live)),
			Some('.') => Some((1, Dead)),
			_ => match u32::from_stream(s) {
				Some((len, n)) => Some((len, Generations(n))),
				None => None,
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
				Live => out_sum;
				*nw,*n,*ne,*w,*e,*sw,*s,*se
			};
			match c {
				Live => if save.contains(&out_sum) {Live} else {Dead},
				Dead => if birth.contains(&out_sum) {Live} else {Dead},
				_ => unreachable!()
			}
		})
	} else {
		Box::new(move |nw: &Cell, n: &Cell, ne: &Cell,
		                w: &Cell, c: &Cell,  e: &Cell,
		               sw: &Cell, s: &Cell, se: &Cell| -> Cell {
			if let Generations(n) = c {
				return
					if n+1 < number {Generations(n+1)}
					else if n+1 == number {Dead}
					else {unreachable!()};
			}
			let mut out_sum: usize = 0;
			count!{$
				Live => out_sum;
				*nw,*n,*ne,*w,*e,*sw,*s,*se
			};
			match c {
				Live => if save.contains(&out_sum) {Live} else {Generations(0)},
				Dead => if birth.contains(&out_sum) {Live} else {Dead},
				_ => unreachable!()
			}
		})
	}
}
#[allow(unreachable_patterns, dead_code)]
pub fn non_totalistic_rule(number: u32, birth: &'static str, save: &'static str) -> BoxRule<Cell> {
	let b_fun = non_totalistic_closure!(Cell; Live, birth);
	let s_fun = non_totalistic_closure!(Cell; Live, save);
	if number == 0 {
		Box::new(move |nw: &Cell, n: &Cell, ne: &Cell,
		                w: &Cell, c: &Cell,  e: &Cell,
		               sw: &Cell, s: &Cell, se: &Cell| -> Cell {
			match c {
				Live => if s_fun(nw,n,ne,w,e,sw,s,se) {Live} else {Dead},
				Dead => if b_fun(nw,n,ne,w,e,sw,s,se) {Live} else {Dead},
				_ => unreachable!()
			}
		})
	} else {
		Box::new(move |nw: &Cell, n: &Cell, ne: &Cell,
		                w: &Cell, c: &Cell,  e: &Cell,
		               sw: &Cell, s: &Cell, se: &Cell| -> Cell {
			match c {
				Live => if s_fun(nw,n,ne,w,e,sw,s,se) {Live} else {Generations(0)},
				Dead => if b_fun(nw,n,ne,w,e,sw,s,se) {Live} else {Dead},
				Generations(n) =>
					if n+1 < number {Generations(n+1)}
					else if n+1 == number {Dead}
					else {unreachable!()}
			}
		})
	}
}