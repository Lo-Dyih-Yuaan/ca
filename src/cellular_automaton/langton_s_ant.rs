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
pub enum State {
	NoAnt(u32),
	NorthAnt(u32),
	EastAnt(u32),
	SouthAnt(u32),
	WestAnt(u32),
}
use self::State::*;
impl State {
	#[inline(always)]
	fn state(&self) -> u32 {
		match self {
			NoAnt(n) => *n,
			NorthAnt(n) => *n,
			EastAnt(n) => *n,
			SouthAnt(n) => *n,
			WestAnt(n) => *n,
		}
	}
}
impl Debug for State {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		write!(f, "{}{}", u32_print_str(self.state()),
			match self {
				NoAnt(_) => "",
				NorthAnt(_) => "^",
				EastAnt(_) => ">",
				SouthAnt(_) => "v",
				WestAnt(_) => "<",
			})
	}
}
impl Display for State {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		let n = self.state();
		let (r, g, b) = hsv2rgb(120*n, 1., 1.);
		write!(f, "\u{1b}[48;2;{};{};{};30m{:<2}\u{1b}[0m",
			r, g, b,
			format!("{}{}",
				u32_print_str(n),
				match self {
					NoAnt(_) => "",
					NorthAnt(_) => "^",
					EastAnt(_) => ">",
					SouthAnt(_) => "v",
					WestAnt(_) => "<",
				}))
	}
}
impl FromStream for State {
	fn from_stream(s: &str) -> Option<(usize, Self)> {
		if let Some((len, n)) = u32::from_stream(s) {
			match (&s[len..]).chars().next() {
				Some('^') => Some((len+1, NorthAnt(n))),
				Some('>') => Some((len+1, EastAnt(n))),
				Some('v') => Some((len+1, SouthAnt(n))),
				Some('<') => Some((len+1, WestAnt(n))),
				_ => Some((len, NoAnt(n)))
			}
		} else {
			None
		}
	}
}

#[allow(dead_code)]
pub fn rule(states: &'static str) -> BoxRule<State> {
	for s in states.chars() {
		if let 'N'|'R'|'L'|'U' = s {}
		else {unreachable!()}
	}
	macro_rules! turn {
		($state:expr, $rotate:expr) => {
			match ($state, $rotate) {
				(NoAnt(n), _) => NoAnt(*n),
				//N，方向不变
				(_, 'N') => $state.clone(),
				//R，向右转
				(NorthAnt(n), 'R') => EastAnt(*n),
				(EastAnt(n), 'R') => SouthAnt(*n),
				(SouthAnt(n), 'R') => WestAnt(*n),
				(WestAnt(n), 'R') => NorthAnt(*n),
				//L，向左转
				(NorthAnt(n), 'L') => WestAnt(*n),
				(EastAnt(n), 'L') => NorthAnt(*n),
				(SouthAnt(n), 'L') => EastAnt(*n),
				(WestAnt(n), 'L') => SouthAnt(*n),
				//U，翻转
				(NorthAnt(n), 'U') => SouthAnt(*n),
				(EastAnt(n), 'U') => WestAnt(*n),
				(SouthAnt(n), 'U') => NorthAnt(*n),
				(WestAnt(n), 'U') => EastAnt(*n),
				_ => unreachable!()
			}
		};
	}
	let states_num: u32 = states.len() as u32;
	let turn_table: Vec<_> = states.chars().collect();
	Box::new(move |_nw: &State, n: &State, _ne: &State,
	                 w: &State, c: &State,   e: &State,
	               _sw: &State, s: &State, _se: &State| -> State {
		let now_state = c.state();
		let next_state = {
			if let NoAnt(_) = c {now_state}
			else if now_state+1 < states_num {now_state+1}
			else if now_state+1 == states_num {0}
			else {unreachable!()}
		};
		let next_n = turn!(n, turn_table[n.state() as usize]);
		let next_e = turn!(e, turn_table[e.state() as usize]);
		let next_s = turn!(s, turn_table[s.state() as usize]);
		let next_w = turn!(w, turn_table[w.state() as usize]);
		let have_ant_from_n = matches!(next_n, SouthAnt(_));
		let have_ant_from_e = matches!(next_e, WestAnt(_));
		let have_ant_from_s = matches!(next_s, NorthAnt(_));
		let have_ant_from_w = matches!(next_w, EastAnt(_));
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
				SouthAnt(next_state)
			} else if have_ant_from_e {
				WestAnt(next_state)
			} else if have_ant_from_s {
				NorthAnt(next_state)
			} else {
				EastAnt(next_state)
			}
		} else {
			NoAnt(next_state)
		}
	})
}