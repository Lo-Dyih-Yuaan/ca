pub trait FromStream: Sized {
	fn from_stream(s: &str) -> Option<(usize, Self)>;
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