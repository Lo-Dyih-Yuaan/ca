mod cellular_automaton;

use std::convert::TryFrom;
use cellular_automaton::Pattern;

macro_rules! from_stream {
	($e:expr) =>
		{<_ as cellular_automaton::FromStream>::from_stream($e).unwrap().1}
}

macro_rules! print_rule_tree {
	($rule:expr, $states:expr) => {{
		let (rf, rs, _) = $rule;
		let mut rt = cellular_automaton::golly::RuleTree::new(
			Pattern::try_from(format!("{}$", $states)).ok().unwrap().get_data()[0].clone());
		rt.create_tree(&rf);
		println!("@RULE {}", rs);
		println!("{:?}", rt);
	}}
}

macro_rules! print_von_neumann_rule_tree {
	($rule:expr, $states:expr) => {{
		let (rf, rs, _) = $rule;
		let mut rt = cellular_automaton::golly::RuleTree::new(
			Pattern::try_from(format!("{}$", $states)).ok().unwrap().get_data()[0].clone());
		rt.create_von_neumann_tree(&rf);
		println!("@RULE {}", rs);
		println!("{:?}", rt);
	}}
}

macro_rules! run_ca {
	($rule:expr, $p:expr, $n:expr) => {
		let (rule, _rule_str, _) = $rule;
		let mut p = Pattern::try_from($p).ok().unwrap();
		for _n in 0..=$n {
			println!("{}第{}代\n", p, _n);
			p = p.tessellate_evolve(&rule);
			
		}
	};
	($rule:expr, $ground:expr, $p:expr, $n:expr) => {
		let (rule, _rule_str, _) = $rule;
		let mut p = (
			Pattern::try_from($p).ok().unwrap(),
			from_stream!($ground),
			0, 0
		);
		let mut x_offset : isize = 0;
		let mut y_offset : isize = 0;
		for _n in 0..=$n {
			println!("{}[{}] ({}, {}) 第{}代\n", p.0, p.1, x_offset, y_offset, _n);
			p = p.0.infinte_evolve(&p.1, &rule);
			x_offset += p.2;
			y_offset += p.3;
		}
	};
}

macro_rules! display_pattern {
	($s:expr, $p:expr) => {
		match $s {
			"{}" => print!("{}", $p),
			"{:x}" => print!("{:x}", $p),
			_ => unreachable!()
		}
	}
}

macro_rules! check {
	(agar $rule:expr, $p:expr, $period:expr) => {
		let (rule, rule_str, rule_display) = $rule;
		let p = Pattern::try_from($p).ok().unwrap();
		display_pattern!(rule_display, p);
		println!("{}", rule_str);
		if p.is_agar(&rule, $period) {
			println!("\u{1b}[32m该图样是{}周期琼脂\u{1b}[0m", $period);
		} else {
			println!("\u{1b}[31m该图样不是{}周期琼脂\u{1b}[0m", $period);
		}
	};
	(still life $rule:expr, $ground:expr, $p:expr) => {
		let (rule, rule_str, rule_display) = $rule;
		let ground = from_stream!($ground);
		let p = Pattern::try_from($p).ok().unwrap();
		display_pattern!(rule_display, p);
		println!("[{}]{}", ground, rule_str);
		if p.is_oscillator(&ground, &rule, 1) {
			println!("\u{1b}[32m该图样是静物\u{1b}[0m", $period);
		} else {
			println!("\u{1b}[31m该图样不是静物\u{1b}[0m", $period);
		}
	};
	(oscillator $rule:expr, $ground:expr, $p:expr, $period:expr) => {
		let (rule, rule_str, rule_display) = $rule;
		let ground = from_stream!($ground);
		let p = Pattern::try_from($p).ok().unwrap();
		display_pattern!(rule_display, p);
		println!("[{}]{}", ground, rule_str);
		if p.is_oscillator(&ground, &rule, $period) {
			println!("\u{1b}[32m该图样是{}周期震荡子\u{1b}[0m", $period);
		} else {
			println!("\u{1b}[31m该图样不是{}周期震荡子\u{1b}[0m", $period);
		}
	};
	(oscillator $rule:expr, $ground:expr, $($name:expr => $p:expr, $period:expr);+ $(;)?) => {
		let (rule, rule_str, rule_display) = $rule;
		let ground = from_stream!($ground);
		$(
			println!("名称：{}", $name);
			let p = Pattern::try_from($p).ok().unwrap();
			display_pattern!(rule_display, p);
			println!("[{}]{}", ground, rule_str);
			if p.is_oscillator(&ground, &rule, $period) {
				println!("\u{1b}[32m该图样是{}周期震荡子\u{1b}[0m", $period);
			} else {
				println!("\u{1b}[31m该图样不是{}周期震荡子\u{1b}[0m", $period);
			}
			println!();
		)+
	};
	(spaceship $rule:expr, $ground:expr, $p:expr, $period:expr) => {
		let (rule, rule_str, rule_display) = $rule;
		let ground = from_stream!($ground);
		let p = Pattern::try_from($p).ok().unwrap();
		display_pattern!(rule_display, p);
		println!("[{}]{}", ground, rule_str);
		let temp = p.is_spaceship(&ground, &rule, $period);
		if let Some((x, y)) = temp {
			println!("\u{1b}[32m该图样是({},{})c/{}飞船\u{1b}[0m", x, y, $period);
		} else {
			println!("\u{1b}[31m该图样不是{}周期飞船\u{1b}[0m", $period);
		}
	};
	(spaceship $rule:expr, $ground:expr, $($name:expr => $p:expr, $period:expr);+ $(;)?) => {
		let (rule, rule_str, rule_display) = $rule;
		let ground = from_stream!($ground);
		$(
			println!("名称：{}", $name);
			let p = Pattern::try_from($p).ok().unwrap();
			display_pattern!(rule_display, p);
			println!("[{}]{}", ground, rule_str);
			let temp = p.is_spaceship(&ground, &rule, $period);
			if let Some((x, y)) = temp {
				println!("\u{1b}[32m该图样是({},{})c/{}飞船\u{1b}[0m", x, y, $period);
			} else {
				println!("\u{1b}[31m该图样不是{}周期飞船\u{1b}[0m", $period);
			}
			println!();
		)+
	};
}

use std::collections::BTreeMap;
macro_rules! map {
	($($k:expr => $v:expr),*) => {{
		let mut temp = BTreeMap::new();
		$(temp.insert($k, $v);)*
		temp
	}};
}

fn from_rle(m: &BTreeMap<char, &str>, ground: &str, rle: &str) -> String {
	//展开游程编码
	let result: String = {
		let mut temp = String::new();
		let mut have_num: bool = false;
		let mut num: usize = 0;
		for c in rle.chars() {
			if let '0'..='9' = c {
				have_num = true;
				num *= 10;
				num += (c as u32 - 0x30) as usize;
			} else if have_num {
				for _ in 0..num {
					temp.push(c);
				}
				have_num = false;
				num = 0;
			} else {
				temp.push(c);
			}
		}
		temp
	};
	//分割
	let result: Vec<String> = {
		let mut temp: Vec<String> = Vec::new();
		let mut line = String::new();
		for c in result.chars() {
			if let '$'|'!' = c {
				temp.push(line);
				line = String::new();
			} else {
				line.push(c);
			}
		}
		temp
	};
	//转换并补全
	let result: Vec<String> = {
		let max_len: usize = result.iter().map(|s| s.len()).max().unwrap();
		let mut temp: Vec<String> = Vec::new();
		for l in result {
			let mut line = String::new();
			for c in l.chars() {
				line.push_str(m.get(&c).unwrap());
			}
			for _ in 0..max_len-l.len() {
				line.push_str(ground);
			}
			temp.push(line);
		}
		temp
	};
	result.into_iter().map(|s| s+"$").fold(String::new(), |x,y| x+&y)
}
fn life_like_from_rle(rle: &str) -> String {
	from_rle(&map!{'b'=>".", 'o'=>"#"}, ".", rle)
}
fn generations_from_rle(rle: &str) -> String {
	from_rle(&map!{'.'=>".", 'A'=>"#", 'B'=>"A", 'C'=>"B", 'D'=>"C", 'E'=>"D", 'F'=>"E", 'G'=>"F", 'H'=>"G", 'I'=>"H", 'J'=>"I", 'K'=>"J", 'L'=>"K", 'M'=>"L", 'N'=>"M", 'O'=>"N", 'P'=>"O", 'Q'=>"P", 'R'=>"Q", 'S'=>"R", 'T'=>"S", 'U'=>"T", 'V'=>"U", 'W'=>"V", 'X'=>"W", 'Y'=>"X", 'Z'=>"Y"}, ".", rle)
}

fn _main() {
	run_ca!{
		rule!{Langton's Ant R L},
		"A",
		concat!{
			"ABAAAAA$",
			"BBBB<AAA$",
			"BBBABAA$",
			"ABAABAA$",
			"ABABABA$",
			"BAABABB$",
			"BABBABA$",
			"ABBBBAA$",
			"AABBAAA$",
		},
		1//104
	};
	run_ca!{
		rule!{B 3 5 7 / S 3 4 5 7 / G 3},
		".",
		concat!{
			".####..####..####.$",
			".####..####..####.$",
			"##BB####BB####BB##$",
			"BA..#CC#..#CC#..AB$",
			".C###CC####CC###C.$",
			"..BAA###CC###AAB..$",
			"...##..####..##...$",
			"...C....##....C...$"
		},
		1//28
	};
	check!{oscillator
		rule!{B 3 5 / S 2 3 4 6 / F 0 1 2 3 4 / K 2 3 / L 0 1 2 3 4},
		".",
		".##$###$###$.##$",
		8
	};
	check!{spaceship
		rule!{B 0 1 3 4 6 8 / S 2 3},
		".",
		"#.$.#$.#$#.$",
		2
	};
	check!{oscillator
		rule!{WireWorld},
		".",
		".#.$@.-$.-.$",
		4
	};
	check!{oscillator
		rule!{Langton's Ant R L},
		"A",
		"A^A^A$AAA$BAA$BBB$",
		40
	};
	check!{oscillator
		rule!{Langton's Ant R L},
		"A",
		"A^A^$",
		28
	};
	check!{oscillator
		rule!{Langton's Ant R L},
		"A",
		"A^A^$BA$",
		16
	};
	check!{spaceship
		rule!{non-totalistic B "2ce3ejnr4aejk5cry6ack" / S "2ci3-acq4aiqrt5-acek78"},
		".",
		"##..$####$#.#.$",
		23
	};
	check!{oscillator
		rule!{non-totalistic B "3e4aqw5y6ak" / S "2cik3ijr4aenqwy5ijk6ce" / G 2},
		".",
		".#.#.$###.#$.#..#$#...#$.###.$",
		11
	};
	check!{oscillator
		rule!{B 3 5 7 / S 3 4 5 7 / G 3},
		".",
		"..#..$.A##.$BC###$.A##.$..#..$",
		26
	};
	check!{agar
		rule!{B 3 / S 2 3},
		concat!{
			"......######$",
			".####.#....#$",
			".#..#.#.##.#$",
			".#..#.#.##.#$",
			".####.#....#$",
			"......######$",
			"######......$",
			"#....#.####.$",
			"#.##.#.#..#.$",
			"#.##.#.#..#.$",
			"#....#.####.$",
			"######......$",
		},
		1
	};
	check!{agar
		rule!{B 3 / S 2 3},
		"...#...$..###..$.#...#.$##...##$.#...#.$..###..$...#...$",
		2
	};
	check!{agar
		rule!{B 3 / S 2 3},
		".#.$#.#$.#.$",
		1
	};

	check!{oscillator rule!{B 3 / S 2 3}, ".",
		"blinker" => "###$", 2;
		"toad" => "#.$##$##$.#$", 2;
		"by flops" => "...#..$.#.#..$.....#$#####.$.....#$.#.#..$...#..$", 2;
		"why not" => "...#...$.#.#...$.....#.$#####.#$.....#.$.#.#...$...#...$", 2;
		"pentadecathlon" => "########$#.####.#$########$", 15;
		"Kok's galaxy" => "######.##$######.##$.......##$##.....##$##.....##$##.....##$##.......$##.######$##.######$", 8;
		"lightweight emulator" => "..##.#..#.##..$..#........#..$...##....##...$###..####..###$#..#......#..#$.##........##.$", 4;
		"middleweight emulator" => ".......#.......$..##.#...#.##..$..#.........#..$...##.....##...$###..#####..###$#..#.......#..#$.##.........##.$", 4;
		"heavyweight emulator" => ".......##.......$..##.#....#.##..$..#..........#..$...##......##...$###..######..###$#..#........#..#$.##..........##.$", 4;
		"David Hilbert" => ".......##...............##.......$........#...............#........$......#...................#......$......#####...........#####......$..........#...........#..........$....####.................####....$....#..#.................#..#....$.....................#...........$.....................#...........$.........#....##.##..#.##........$........###...##.#......##.......$.......#..##......#..............$.......###............#..........$......................#.#........$...##...........##....#..#..##...$...#............##.....##....#...$##.#............##...........#.##$#.##.##...................##.##.#$.....#.....................#.....$.....#.#.................#.#.....$......##.................##......$..........#...........#..........$......#####...........#####......$......#...................#......$........#...............#........$.......##...............##.......$", 23
	};
	check!{spaceship rule!{B 3 / S 2 3}, ".",
		"glider" => ".#.$..#$###$", 4;
		"lightweight spaceship(LWSS)" => "#..#.$....#$#...#$.####$", 4;
		"middleweight spaceship(MWSS)" => "..#...$#...#.$.....#$#....#$.#####$", 4;
		"heavyweight spaceship(HWSS)" => "..##...$#....#.$......#$#.....#$.######$", 4;
		"Schick engine" => "...........####$..........#...#$......#.......#$.#######..#..#.$#..###.##......$.#######..#..#.$......#.......#$..........#...#$...........####$", 12;
		"big A" => ".....####$....#...#$........#$....#..#.$.........$####.....$.#..#....$.#..#....$.#..#....$####.....$.........$....#..#.$........#$....#...#$.....####$", 4;
		"copperhead" => "....###.##...$.#....##..##.$##.....#....#$##.....#....#$.#....##..##.$....###.##...$", 10;
		"loafer" => "##.#..##.$.##..#..#$.....#.#.$......#..$#........$###......$...#.....$..#......$##.......$", 7;
		"turtle" => "#.......###.$##.##.#..##.$.#....###...$.#...#.#..#.$.#....#....#$.#....#....#$.#...#.#..#.$.#....###...$##.##.#..##.$#.......###.$", 3;
		"dart" => ".#........$#.#.......$..##......$#.........$#...#.....$....#..#..$####..#.#.$......#..#$####..#.#.$....#..#..$#...#.....$#.........$..##......$#.#.......$.#........$", 3;
		"Sir Robin" => include_str!("Sir Robin/Sir Robin.pattern"), 6;
		"Sir Robin with errant minstrel" => include_str!("Sir Robin/Sir Robin with errant minstrel.pattern"), 6;
		"Sir Robin with lightweight minstrel" => include_str!("Sir Robin/Sir Robin with lightweight minstrel.pattern"), 6;
		"Sir Robin with heavyweight minstrel" => include_str!("Sir Robin/Sir Robin with heavyweight minstrel.pattern"), 6;
		"Sir Robin with heavyweight-errant minstrel" => include_str!("Sir Robin/Sir Robin with heavyweight-errant minstrel.pattern"), 6;
		"Sir Robin with heavyweight-wandering minstrel" => include_str!("Sir Robin/Sir Robin with heavyweight-wandering minstrel.pattern"), 6;
		"Sir Robin with lightweight-ragged minstrel" => include_str!("Sir Robin/Sir Robin with lightweight-ragged minstrel.pattern"), 6;
		"Sir Robin with lightweight-ragged-errant minstrel" => include_str!("Sir Robin/Sir Robin with lightweight-ragged-errant minstrel.pattern"), 6;
		"Sir Robin with lightweight-ragged-wandering minstrel" => include_str!("Sir Robin/Sir Robin with lightweight-ragged-wandering minstrel.pattern"), 6;
		"Sir Robin with lightweight-ragged-featherweight minstrel" => include_str!("Sir Robin/Sir Robin with lightweight-ragged-featherweight minstrel.pattern"), 6;
		"Sir Robin with heavyweight-pi minstrel" => include_str!("Sir Robin/Sir Robin with heavyweight-pi minstrel.pattern"), 6;
		"Sir Robin with heavyweight-wandering-connecting minstrel" => include_str!("Sir Robin/Sir Robin with heavyweight-wandering-connecting minstrel.pattern"), 6;
		"Sir Robin with errant-fountain minstrel" => include_str!("Sir Robin/Sir Robin with errant-fountain minstrel.pattern"), 6;
		"Sir Robin with lightweight-ragged-pi minstrel" => include_str!("Sir Robin/Sir Robin with lightweight-ragged-pi minstrel.pattern"), 6;
		"Sir Robin with lightweight-ragged-wandering-connecting minstrel" => include_str!("Sir Robin/Sir Robin with lightweight-ragged-wandering-connecting minstrel.pattern"), 6;
		"Sir Robin with heavyweight-errant-fountain minstrel" => include_str!("Sir Robin/Sir Robin with heavyweight-errant-fountain minstrel.pattern"), 6;
		"Sir Robin with lightweight-ragged-errant-fountain minstrel" => include_str!("Sir Robin/Sir Robin with lightweight-ragged-errant-fountain minstrel.pattern"), 6;
	};
	check!{oscillator
		rule!{B 2 / S 3 4 H}, ".",
		"....#..........$..#.#..#.......$.#..##.#.......$....#.#...#....$###########....$..#.#...#.#....$...##....##.#..$.##.#.....#....$....##....##.#.$....#.#...#.#..$...###########.$........#.#...#$......#..##.#..$........#.#..#.$...........#...$",
		6
	};
	check!{oscillator
		rule!{B 2 / S 3 4 H}, ".",
		"#....#.$#.#.#.#$..####.$....#..$",
		6
	};
	check!{oscillator
		rule!{B 2 / S 3 4 H}, ".",
		"##.$#.#$.##$",
		3
	};
	
	check!{oscillator
		rule!{B 3 / S 2 3 / G 6}, ".",
		generations_from_rle("2.A$.3A$.AFCA$3.2CA$.2FCA$EFCB$.D!"),
		32
	};
	check!{oscillator
		rule!{B 3 / S 2 3 / G 7}, ".",
		generations_from_rle("3.A$2.3A$.HAFCA$3.H2CA$2.2FCA$.EFCB$H.D!"),
		32
	};
	check!{oscillator
		rule!{B 3 / S 2 3 / G 8}, ".",
		generations_from_rle("4.A$3.3A$2.HAFCA$3.IH2CA$3.2FCA$I.EFCB$.H.D!"),
		32
	};
	check!{oscillator
		rule!{B 3 / S 2 3 / G 9}, ".",
		generations_from_rle("5.A$4.3A$3.HAFCA$4.IH2CA$J3.2FCA$.I.EFCB$2.H.D!"),
		32
	};
	check!{oscillator
		rule!{B 3 / S 2 3 / G 10}, ".",
		generations_from_rle("5.A$4.3A$3.HAFCA$K3.IH2CA$J3K2FCA$.IKEFCB$2.H.D!"),
		32
	};
	check!{oscillator
		rule!{B 3 / S 2 3 / G 11}, ".",
		generations_from_rle("6.A$5.3A$L3.HAFCA$.K3.IH2CA$.J3K2FCA$2.IKEFCB$3.H.D!"),
		32
	};
	check!{oscillator
		rule!{B 3 / S 2 3 / G 12}, ".",
		generations_from_rle("6.A$.M3.3A$L3.HAFCA$.K3.IH2CA$.J3K2FCA$2.IKEFCB$3.H.D!"),
		32
	};
	check!{oscillator
		rule!{B 3 / S 2 3 / G 13}, ".",
		generations_from_rle("6.A$.M3.3A$L2N.HAFCA$.KN.NIH2CA$.J3K2FCA$2.IKEFCB$3.H.D!"),
		32
	};
	check!{oscillator
		rule!{B 3 / S 2 3 / G 14}, ".",
		generations_from_rle("6.A$.M3.3A$L2N.HAFCA$.KN.NIH2CA$.J3K2FCA$2.IKEFCB$3.H.D!"),
		32
	};
	check!{oscillator
		rule!{B 3 / S 2 3 / G 15}, ".",
		generations_from_rle("P5.A$.M2.P3A$L2N.HAFCA$.KNPNIH2CA$.J3K2FCA$2.IKEFCB$3.H.D!"),
		32
	};
	/*let temp = [
		"2.A$.3A$.AFCA$3.2CA$.2FCA$EFCB$.D!",
		"3.A$2.3A$.HAFCA$3.H2CA$2.2FCA$.EFCB$H.D!",
		"4.A$3.3A$2.HAFCA$3.IH2CA$3.2FCA$I.EFCB$.H.D!",
		"5.A$4.3A$3.HAFCA$4.IH2CA$J3.2FCA$.I.EFCB$2.H.D!",
		"5.A$4.3A$3.HAFCA$K3.IH2CA$J3K2FCA$.IKEFCB$2.H.D!",
		"6.A$5.3A$L3.HAFCA$.K3.IH2CA$.J3K2FCA$2.IKEFCB$3.H.D!",
		"6.A$.M3.3A$L3.HAFCA$.K3.IH2CA$.J3K2FCA$2.IKEFCB$3.H.D!",
		"6.A$.M3.3A$L2N.HAFCA$.KN.NIH2CA$.J3K2FCA$2.IKEFCB$3.H.D!",
		"6.A$.M3.3A$L2N.HAFCA$.KN.NIH2CA$.J3K2FCA$2.IKEFCB$3.H.D!",
		"P5.A$.M2.P3A$L2N.HAFCA$.KNPNIH2CA$.J3K2FCA$2.IKEFCB$3.H.D!"
	];
	for n in 0..10 {
		let rule = cellular_automaton::generations::rule(
			std::num::NonZeroU32::new(n as u32 + 6).unwrap(),
			&[3], &[2,3]);
		let ground = from_stream!(".");
		let mut p = Pattern::try_from(generations_from_rle(temp[n])).ok().unwrap();
		for _ in 0..16 {
			p = p.infinte_evolve(&ground, &rule).0;
		}
		println!("{}", p);
	}*/
	check!{oscillator
		rule!{von Neumann 29}, "U",
		"To>~Tov_$To^_To<_$",
		4
	};
	run_ca!{
		rule!{von Neumann 29},
		"U",
		concat!{
			"SS0S1S00S01S10S11S000$",
			"To>_To^_To<_Tov_To>~To^~To<~Tov~$",
			"Ts>_Ts^_Ts<_Tsv_Ts>~Ts^~Ts<~Tsv~$",
			"UUUUUUUU$",
			"C__UC_~UC~_UC~~U$",
		},
		1
	};
	/*run_ca!{
		rule!{NoTimeAtAll},
		".",
		concat!{
			"1---......$",
			"...-------$",
			"0---......$",
		},
		12
	};
	run_ca!{
		rule!{NoTimeAtAll},
		".",
		concat!{
			"...-...$",
			"1---...$",
			"...-...$",
		},
		5
	};
	run_ca!{
		rule!{NoTimeAtAll},
		".",
		concat!{
			"...-...$",
			"1------$",
		},
		5
	};
	run_ca!{
		rule!{NoTimeAtAll},
		".",
		concat!{
			"...---...$",
			"1---.----$",
			"...---...$",
			"0---.----$",
			"...---...$",
		},
		12
	};*/
	/*print_rule_tree!(
		rule!{B 3 4 / S 2 3 4 / F 0 1 2 3 6 / K 2 3 4 5 / L 0 2 3 4 5},
		".#@"
	);*/
	/*print_von_neumann_rule_tree!(
		rule!{Nobili 32},
		concat!{
			"USS0S1S00S01S10S11S000",
			"To>_To^_To<_Tov_",
			"To>~To^~To<~Tov~",
			"Ts>_Ts^_Ts<_Tsv_",
			"Ts>~Ts^~Ts<~Tsv~",
			"C__C_~C~_C~~C-C|C+"
		}
	);*/
	check!{spaceship rule!{non-totalistic B "2o3-o4m"/S "12m3o4m5" H}, ".",
		"##$##$#.$##$.#$",
		65
	}
	check!{spaceship rule!{B 2 4 / S 1 3 / G 2 H}, ".",
		concat!{
			".#....$",
			"#A##..$",
			".#.BA.$",
			".#BA#.$",
			".#B..B$",
			"..A#..$",
			"..#.#.$",
			"...BAB$",
			"....##$"
		},
		14
	}
	check!{spaceship rule!{non-totalistic B "24" / S "134m" / G 2 H}, ".",
		concat!{
			"A###A...$",
			"#BAAB#..$",
			".#ABA#..$",
			"B.#BB#.B$",
		},
		42
	}
}

fn main() {
	print_von_neumann_rule_tree!(
		rule!{Hutton 32},
		concat!{
			"USS0S1S00S01S10S11S000",
			"To>_To^_To<_Tov_",
			"To>~To^~To<~Tov~",
			"Ts>_Ts^_Ts<_Tsv_",
			"Ts>~Ts^~Ts<~Tsv~",
			"C__C_~C~_C~~C-C|C+"
		}
	);
}