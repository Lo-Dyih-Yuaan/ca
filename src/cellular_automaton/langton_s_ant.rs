use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use super::u32_print_str;
use super::FromStream;
use super::BoxRule;

fn hsv2rgb(h: u32, s: f64, v: f64) -> (u8, u8, u8) {
	let h: u32 = h % 360;
	let c = v * s;
	let x = c * (1. - f64::abs((h as f64/60.) % 2. - 1.));
	let m = v - c;
	let (r_, g_, b_): (f64, f64, f64) = match h / 60 {
		0 => (c, x, 0.),
		1 => (x, c, 0.),
		2 => (0., c, x),
		3 => (0., x, c),
		4 => (x, 0., c),
		5 => (c, 0., x),
		_ => (0., 0., 0.)
	};
	let r: u8 = ((r_ + m)*255.) as u8;
	let g: u8 = ((g_ + m)*255.) as u8;
	let b: u8 = ((b_ + m)*255.) as u8;
	(r, g, b)
}

#[derive(PartialEq, Eq, Clone)]
pub enum Cell {
	NoAnt(u32),
	NorthAnt(u32),
	EastAnt(u32),
	SouthAnt(u32),
	WestAnt(u32),
}
impl Cell {
	#[inline(always)]
	fn state(&self) -> u32 {
		match self {
			Cell::NoAnt(n) => *n,
			Cell::NorthAnt(n) => *n,
			Cell::EastAnt(n) => *n,
			Cell::SouthAnt(n) => *n,
			Cell::WestAnt(n) => *n,
		}
	}
}
impl Debug for Cell {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		write!(f, "{}", u32_print_str(self.state()))?;
		match self {
			Cell::NoAnt(_) => write!(f, ""),
			Cell::NorthAnt(_) => write!(f, "^"),
			Cell::EastAnt(_) => write!(f, ">"),
			Cell::SouthAnt(_) => write!(f, "v"),
			Cell::WestAnt(_) => write!(f, "<"),
		}
	}
}
impl Display for Cell {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		let n = self.state();
		let (r, g, b) = hsv2rgb(180*n, 1., 1.);
		write!(f, "\u{1b}[48;2;{};{};{}m{}", r, g, b, u32_print_str(n))?;
		write!(f, "{}", match self {
			Cell::NoAnt(_) => " \u{1b}[0m",
			Cell::NorthAnt(_) => "\u{1b}[0m^",
			Cell::EastAnt(_) => "\u{1b}[0m>",
			Cell::SouthAnt(_) => "\u{1b}[0mv",
			Cell::WestAnt(_) => "\u{1b}[0m<",
		})
	}
}
impl FromStream for Cell {
	fn from_stream(s: &str) -> (usize, Option<Self>) {
		let (len, option_n) = u32::from_stream(s);
		if let Some(n) = option_n {
			match (&s[len..]).chars().next() {
				Some('^') => (len+1, Some(Cell::NorthAnt(n))),
				Some('>') => (len+1, Some(Cell::EastAnt(n))),
				Some('v') => (len+1, Some(Cell::SouthAnt(n))),
				Some('<') => (len+1, Some(Cell::WestAnt(n))),
				_ => (len, Some(Cell::NoAnt(n)))
			}
		} else {
			(len, None)
		}
	}
}

#[allow(dead_code)]
pub fn rule(states: &'static str) -> BoxRule<Cell> {
	for s in states.chars() {
		if let 'N'|'R'|'L'|'U' = s {}
		else {unreachable!()}
	}
	macro_rules! turn {
		($state:expr, $rotate:expr) => {
			match ($state, $rotate) {
				(Cell::NoAnt(n), _) => Cell::NoAnt(*n),
				//N，方向不变
				(_, Some('N')) => $state.clone(),
				//R，向右转
				(Cell::NorthAnt(n), Some('R')) => Cell::EastAnt(*n),
				(Cell::EastAnt(n), Some('R')) => Cell::SouthAnt(*n),
				(Cell::SouthAnt(n), Some('R')) => Cell::WestAnt(*n),
				(Cell::WestAnt(n), Some('R')) => Cell::NorthAnt(*n),
				//L，向左转
				(Cell::NorthAnt(n), Some('L')) => Cell::WestAnt(*n),
				(Cell::EastAnt(n), Some('L')) => Cell::NorthAnt(*n),
				(Cell::SouthAnt(n), Some('L')) => Cell::EastAnt(*n),
				(Cell::WestAnt(n), Some('L')) => Cell::SouthAnt(*n),
				//U，翻转
				(Cell::NorthAnt(n), Some('U')) => Cell::SouthAnt(*n),
				(Cell::EastAnt(n), Some('U')) => Cell::WestAnt(*n),
				(Cell::SouthAnt(n), Some('U')) => Cell::NorthAnt(*n),
				(Cell::WestAnt(n), Some('U')) => Cell::EastAnt(*n),
				_ => unreachable!()
			}
		};
	}
	let states_num: u32 = states.len() as u32;
	Box::new(move |_nw: &Cell, n: &Cell, _ne: &Cell,
	                 w: &Cell, c: &Cell,   e: &Cell,
	               _sw: &Cell, s: &Cell, _se: &Cell| -> Cell {
		let now_state = c.state();
		let next_state = {
			if let Cell::NoAnt(_) = c {now_state}
			else if now_state+1 < states_num {now_state+1}
			else if now_state+1 == states_num {0}
			else {unreachable!()}
		};
		let next_n = turn!(n, states.chars().nth(n.state() as usize));
		let next_e = turn!(e, states.chars().nth(e.state() as usize));
		let next_s = turn!(s, states.chars().nth(s.state() as usize));
		let next_w = turn!(w, states.chars().nth(w.state() as usize));
		let have_ant_from_n = if let Cell::SouthAnt(_) = next_n {true} else {false};
		let have_ant_from_e = if let Cell::WestAnt(_) = next_e {true} else {false};
		let have_ant_from_s = if let Cell::NorthAnt(_) = next_s {true} else {false};
		let have_ant_from_w = if let Cell::EastAnt(_) = next_w {true} else {false};
		let ants_num = {
			let mut temp = 0;
			if have_ant_from_n {temp += 1;}
			if have_ant_from_e {temp += 1;}
			if have_ant_from_s {temp += 1;}
			if have_ant_from_w {temp += 1;}
			temp
		};
		if ants_num == 1 {
			if have_ant_from_n {
				Cell::SouthAnt(next_state)
			} else if have_ant_from_e {
				Cell::WestAnt(next_state)
			} else if have_ant_from_s {
				Cell::NorthAnt(next_state)
			} else {
				Cell::EastAnt(next_state)
			}
		} else {
			Cell::NoAnt(next_state)
		}
	})
}