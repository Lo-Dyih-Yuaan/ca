use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::LowerHex;
use std::fmt::Formatter;
use std::str::FromStr;
use std::convert::TryFrom;
use crate::rules::RuleType;
use crate::rules::from_stream::FromStream;

#[derive(Clone)]
pub struct Pattern<T> {
	data: Vec<Vec<T>>
}
impl<T> Pattern<T> {
	fn new() -> Self {
		Pattern { data: Vec::<Vec<T>>::new() }
	}
	fn is_pattern(&self) -> bool {
		let len = self.data.first().map(|v| v.len());
		self.data.is_empty() || self.data.iter().all(|v| v.len() == len.unwrap())
	}
	fn is_empty(&self) -> bool {
		self.data.is_empty() || self.data.iter().all(|v| v.is_empty())
	}
}
impl<T: Clone> Pattern<T> {
	pub fn set(&mut self, i: usize, j: usize, state: &T) {
		self.data[i][j] = state.clone();
	}
	pub fn get(&self, i: usize, j: usize) -> T {
		self.data[i][j].clone()
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
	pub fn get_data(&self) -> Vec<Vec<T>> {
		self.data.clone()
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

impl<T> FromStr for Pattern<T>
  where T: FromStream + Clone {
	type Err = Vec<Vec<T>>;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut p: Pattern<T> = Pattern::new();
		let mut temp_line: Vec<T> = Vec::new();
		let mut temp_str = s;
		while !temp_str.is_empty() {
			let r = T::from_stream(temp_str);
			if let Some((len, ele)) = r {
				temp_line.push(ele);
				temp_str = &temp_str[len..];
			} else {
				p.data.push(temp_line.clone());
				temp_line.clear();
				let mut chars = temp_str.chars();
				chars.next();
				temp_str = chars.as_str();
			}
		}
		if p.is_pattern() {
			if p.is_empty() {
				p.data.clear();
			}
			Result::Ok(p)
		} else {
			Result::Err(p.data)
		}
	}
}

impl<T> TryFrom<&str> for Pattern<T>
  where T: FromStream + Clone {
	type Error = Vec<Vec<T>>;
	fn try_from(s: &str) -> Result<Self, Self::Error> {
		s.parse()
	}
}
impl<T> TryFrom<String> for Pattern<T>
  where T: FromStream + Clone {
	type Error = Vec<Vec<T>>;
	fn try_from(s: String) -> Result<Self, Self::Error> {
		s.as_str().parse()
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