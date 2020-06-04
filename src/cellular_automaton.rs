use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::LowerHex;
use std::fmt::Formatter;
use std::convert::TryFrom;

pub trait RuleType<T> : Fn(&T, &T, &T, &T, &T, &T, &T, &T, &T) -> T {}
impl<T, F: Fn(&T, &T, &T, &T, &T, &T, &T, &T, &T) -> T> RuleType<T> for F {}
type BoxRule<T> = Box<dyn Fn (&T, &T, &T, &T, &T, &T, &T, &T, &T) -> T>;

#[allow(dead_code)]
pub mod golly;

/**
 * 用于计数。
 * `$d`必为`$`，以解决宏内`$`将被转义的问题。
 * `$m`为统计对象；`$i`为相应的匹配次数，应当是可变的，类型建议为`usize`。
 * `$e`为待统计对象。
 * 匹配使用`==`。
 */
macro_rules! count {
	($d:tt $($m:expr => $i:ident),+ ; $($e:expr),*) => {
		macro_rules! __count {
			($d arg:expr) => {
				$(if $d arg == $m {
					$i += 1;
				} else)+ { (); }
			};
		}
		$(__count!($e);)*
	};
}
#[macro_use]
mod symmetry;
#[inline(always)]
fn u32_print_str(n: u32) -> String {
	if n < 26 {
		std::char::from_u32(0x41u32+n).unwrap().to_string()
	} else {
		let mut s = String::from("[");
		s.push_str(n.to_string().as_str());
		s.push(']');
		s
	}
}
pub trait FromStream: Sized {
	fn from_stream(&str) -> Option<(usize, Self)>;
}
impl FromStream for u32 {
	#[inline(always)]
	fn from_stream(s: &str) -> Option<(usize, Self)> {
		let mut chars = s.chars();
		match chars.next() {
			Some(c @ 'A' ..= 'Z') =>
				Some((1, u32::from(c)-0x41)),
			Some('[') => {
				let mut n: u32 = 0;
				let mut len: usize = 0; 
				for c in chars {
					if c == ']' {
						return Some((len+2, n));
					} else if let '0' ..= '9' = c {
						len += 1;
						n *= 10;
						n += u32::from(c)-0x30;
					} else { return None; }
				}
				None
			},
			_ => None,
		}
	}
}

pub mod life;
pub mod generations;
pub mod wireworld;
pub mod no_time_at_all;
pub mod langton_s_ant;
pub mod bsfkl;
pub mod von_neumann29;

#[derive(Clone)]
pub struct Pattern<T> {
	data: Vec<Vec<T>>
}
impl<T> Pattern<T> {
	fn new() -> Self {
		Pattern { data: Vec::<Vec<T>>::new() }
	}
	fn is_empty(&self) -> bool {
		self.data.is_empty() || self.data[0].is_empty()
	}
}
#[allow(dead_code)]
impl<T> Pattern<T>
  where T: Clone + PartialEq {
	pub fn tessellate_evolve<F>(&self, f: F) -> Self
	  where F: RuleType<T> {
		let mut p: Self = Pattern::new();
		if self.is_empty() {
			p
		} else {
			let x_len = self.data[0].len();
			let y_len = self.data.len();
			for y in 0..y_len {
				let mut temp: Vec<T> = Vec::new();
				for x in 0..x_len {
					let n_index = if y == 0 {y_len-1} else {y-1};
					let s_index = if y == y_len-1 {0} else {y+1};
					let w_index = if x == 0 {x_len-1} else {x-1};
					let e_index = if x == x_len-1 {0} else {x+1};
					let nw: &T = &self.data[n_index][w_index];
					let  n: &T = &self.data[n_index][x];
					let ne: &T = &self.data[n_index][e_index];
					let  w: &T = &self.data[y][w_index];
					let  c: &T = &self.data[y][x];
					let  e: &T = &self.data[y][e_index];
					let sw: &T = &self.data[s_index][w_index];
					let  s: &T = &self.data[s_index][x];
					let se: &T = &self.data[s_index][e_index];
					temp.push(f(
						nw, n, ne,
						 w, c,  e,
						sw, s, se
					));
				}
				p.data.push(temp);
			}
			p
		}
	}
	pub fn infinte_evolve<F>(&self, ground: &T, f: &F) -> (Self, T, isize, isize)
	  where F: RuleType<T> {
		let mut p: Self = Pattern::new();
		let next_ground =
			f(ground, ground, ground,
			  ground, ground, ground,
			  ground, ground, ground);
		if self.is_empty() {
			(p, next_ground, 0, 0)
		} else {
			let x_len = self.data[0].len();
			let y_len = self.data.len();
			let mut ground_line = Vec::new() as Vec<T>;
			ground_line.resize(x_len, ground.clone());
			macro_rules! next_line {
				($nl: expr, $cl: expr, $sl: expr) => {
					{
						let mut temp = Vec::new() as Vec<T>;
						temp.push(f(
							ground, ground, $nl.first().unwrap(),
							ground, ground, $cl.first().unwrap(),
							ground, ground, $sl.first().unwrap()
						));
						for x in 0..x_len {
							let has_w = x != 0;
							let has_e = x != x_len-1;
							let nw = if has_w {&$nl[x-1]} else {&ground};
							let  n = &$nl[x];
							let ne = if has_e {&$nl[x+1]} else {&ground};
							let  w = if has_w {&$cl[x-1]} else {&ground};
							let  c = &$cl[x];
							let  e = if has_e {&$cl[x+1]} else {&ground};
							let sw = if has_w {&$sl[x-1]} else {&ground};
							let  s = &$sl[x];
							let se = if has_e {&$sl[x+1]} else {&ground};
							temp.push(f(
								nw, n, ne,
								 w, c,  e,
								sw, s, se
							));
						}
						temp.push(f(
							$nl.last().unwrap(), ground, ground,
							$cl.last().unwrap(), ground, ground,
							$sl.last().unwrap(), ground, ground,
						));
						temp
					}
				}
			}
			p.data.push(next_line!(ground_line, ground_line, self.data.first().unwrap()));
			for y in 0..y_len {
				let nl = if y != 0 {&self.data[y-1]} else {&ground_line};
				let sl = if y != y_len-1 {&self.data[y+1]} else {&ground_line};
				p.data.push(next_line!(nl, &self.data[y], sl))
			}
			p.data.push(next_line!(self.data.last().unwrap(), ground_line, ground_line));
			let (x_offset, y_offset) = p.simpify(&next_ground);
			(p, next_ground, x_offset-1, y_offset-1)
		}
	}
	fn simpify(&mut self, ground: &T) -> (isize, isize) {
		while !self.is_empty() &&
		  self.data.last().unwrap().iter().all(|x| x == ground) {
			self.data.pop();
		}
		while !self.is_empty() &&
		  self.data.iter().map(|v| v.last().unwrap()).all(|x| x == ground) {
			for v in &mut self.data {
				v.pop();
			}
		}
		let mut y_offset: isize = 0;
		while !self.is_empty() &&
		  self.data.first().unwrap().iter().all(|x| x == ground) {
			self.data.remove(0);
			y_offset += 1;
		}
		let mut x_offset: isize = 0;
		while !self.is_empty() &&
		  self.data.iter().map(|v| v.first().unwrap()).all(|x| x == ground) {
			for v in &mut self.data {
				v.remove(0);
			}
			x_offset += 1;
		}
		(x_offset, y_offset)
	}
}
impl<T> Debug for Pattern<T>
  where T: Debug {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		for l in self.data.iter() {
			for e in l.iter() {
				write!(f, "{:?}", e)?;
			}
			writeln!(f)?;
		}
		Ok(())
	}
}
impl<T> Display for Pattern<T>
  where T: Display {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		for l in self.data.iter() {
			for e in l.iter() {
				write!(f, "{}", e)?;
			}
			writeln!(f)?;
		}
		Ok(())
	}
}
impl<T> LowerHex for Pattern<T>
  where T: Display {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		let len = self.data.len();
		let mut s = String::new();
		for _ in 0..len {
			s.push(' ');
		}
		let mut s = s.as_str();
		for l in self.data.iter() {
			s = &s[1..];
			write!(f, "{}", s)?;
			for e in l.iter() {
				write!(f, "{}", e)?;
			}
			writeln!(f)?;
		}
		Ok(())
	}
}


impl<T> TryFrom<&str> for Pattern<T>
  where T: FromStream + Clone {
	type Error = Vec<Vec<T>>;
	fn try_from(s: &str) -> Result<Self, Self::Error> {
		let mut p: Pattern<T> = Pattern::new();
		let mut temp: Vec<T> = Vec::new();
		let mut temp_str = s;
		while !temp_str.is_empty() {
			let r = T::from_stream(temp_str);
			if let Some((len, ele)) = r {
				temp.push(ele);
				temp_str = &temp_str[len..];
			} else {
				p.data.push(temp.clone());
				temp.clear();
				let mut chars = temp_str.chars();
				chars.next();
				temp_str = chars.as_str();
			}
		}
		if p.is_empty() {
			Result::Ok(p)
		} else if p.data.iter().all(|v| v.is_empty()) {
			p.data.clear();
			Result::Ok(p)
		} else {
			let len = p.data[0].len();
			if p.data.iter().all(|v| v.len() == len) {
				Result::Ok(p)
			} else {
				Result::Err(p.data)
			}
		}
	}
}
impl<T> TryFrom<String> for Pattern<T>
  where T: FromStream + Clone {
	type Error = Vec<Vec<T>>;
	fn try_from(s: String) -> Result<Self, Self::Error> {
		<Self as TryFrom<&str>>::try_from(&s)
	}
}

impl<T: Eq+Clone> Pattern<T> {
	pub fn is_oscillator<F>(&self, ground: &T, f: &F, period: usize) -> bool
	  where F: RuleType<T> {
		let mut temp = (
			self.clone(),
			ground.clone(),
			0isize, 0isize
		);
		let mut x_offset : isize = 0;
		let mut y_offset : isize = 0;
		for _n in 0..period {
			temp = temp.0.infinte_evolve(&temp.1, f);
			x_offset += temp.2;
			y_offset += temp.3;
			if x_offset == 0 && y_offset == 0 && temp.0.data == self.data && temp.1 == *ground {
				return _n+1 == period;
			}
		}
		false
	}
	pub fn is_spaceship<F>(&self, ground: &T, f: &F, period: usize) -> Option<(isize, isize)>
	  where F: RuleType<T> {
		let mut temp = (
			self.clone(),
			ground.clone(),
			0isize, 0isize
		);
		let mut x_offset : isize = 0;
		let mut y_offset : isize = 0;
		for _n in 0..period {
			temp = temp.0.infinte_evolve(&temp.1, f);
			x_offset += temp.2;
			y_offset += temp.3;
			if !(x_offset == 0 && y_offset == 0) && temp.0.data == self.data && temp.1 == *ground {
				return if _n+1 == period {Some((x_offset, y_offset))} else {None};
			}
		}
		None
	}
	pub fn is_agar<F>(&self, f: &F, period: usize) -> bool
	  where F: RuleType<T> {
		let mut temp = self.clone();
		for _n in 0..period {
			temp = temp.tessellate_evolve(f);
			if temp.data == self.data {
				return _n+1 == period;
			}
		}
		false
	}
}