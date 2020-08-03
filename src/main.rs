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

fn main() {
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
		/*"Sir Robin" => life_like_from_rle("4b2o$4bo2bo$4bo3bo$6b3o$2b2o6b4o$2bob2o4b4o$bo4bo6b3o$2b4o4b2o3bo$o9b2o$bo3bo$6b3o2b2o2bo$2b2o7bo4bo$13bob2o$10b2o6bo$11b2ob3obo$10b2o3bo2bo$10bobo2b2o$10bo2bobobo$10b3o6bo$11bobobo3bo$14b2obobo$11bo6b3o2$11bo9bo$11bo3bo6bo$12bo5b5o$12b3o$16b2o$13b3o2bo$11bob3obo$10bo3bo2bo$11bo4b2ob3o$13b4obo4b2o$13bob4o4b2o$19bo$20bo2b2o$20b2o$21b5o$25b2o$19b3o6bo$20bobo3bobo$19bo3bo3bo$19bo3b2o$18bo6bob3o$19b2o3bo3b2o$20b4o2bo2bo$22b2o3bo$21bo$21b2obo$20bo$19b5o$19bo4bo$18b3ob3o$18bob5o$18bo$20bo$16bo4b4o$20b4ob2o$17b3o4bo$24bobo$28bo$24bo2b2o$25b3o$22b2o$21b3o5bo$24b2o2bobo$21bo2b3obobo$22b2obo2bo$24bobo2b2o$26b2o$22b3o4bo$22b3o4bo$23b2o3b3o$24b2ob2o$25b2o$25bo2$24b2o$26bo!"), 6;
		"Sir Robin with errant minstrel" => life_like_from_rle("4b2o$4bo2bo$4bo3bo$6b3o$2b2o6b4o$2bob2o4b4o$bo4bo6b3o$2b4o4b2o3bo$o9b2o$bo3bo$6b3o2b2o2bo$2b2o7bo4bo$13bob2o$10b2o6bo$11b2ob3obo$10b2o3bo2bo$10bobo2b2o$10bo2bobobo$10b3o6bo$11bobobo3bo$14b2obobo$11bo6b3o2$11bo9bo$11bo3bo6bo$12bo5b5o$12b3o$16b2o$13b3o2bo$11bob3obo$10bo3bo2bo$11bo4b2ob3o$13b4obo4b2o$13bob4o4b2o$19bo$20bo2b2o$20b2o$21b5o$25b2o$19b3o6bo$20bobo3bobo$19bo3bo3bo$19bo3b2o$18bo6bob3o$19b2o3bo3b2o$20b4o2bo2bo$22b2o3bo$21bo$21b2obo$20bo$19b5o$19bo4bo$18b3ob3o$18bob5o$18bo$20bo$16bo4b4o$20b4ob2o$17b3o4bo$24bobo$28bo$24bo2b2o$25b3o$22b2o$21b3o5bo$24b2o2bobo$21bo2b3obobo$22b2obo2bo$24bobo2b2o$26b2o$22b3o4bo$22b3o4bo$23b2o3b3o$24b2ob2o$25b2o$25bo2$24b2o4b3o$26bo5b2o$33bo$35bo$31bo4bo$37bo$31bo4bo2b2o$30b2ob3o3bo$32bob2o$30bobob2o$31bo3b2o$34b2o$35b2o$34bo$33bo2bo$33bo$32bo3bo$33b2ob2o$34bobo$30b3o3bo$30b2o3b2o$32b2obob2o$34bo$31bo$31bo!"), 6;
		"Sir Robin with lightweight minstrel" => life_like_from_rle("4b2o$4bo2bo$4bo3bo$6b3o$2b2o6b4o$2bob2o4b4o$bo4bo6b3o$2b4o4b2o3bo$o9b2o$bo3bo$6b3o2b2o2bo$2b2o7bo4bo$13bob2o$10b2o6bo$11b2ob3obo$10b2o3bo2bo$10bobo2b2o$10bo2bobobo$10b3o6bo$11bobobo3bo$14b2obobo$11bo6b3o2$11bo9bo$11bo3bo6bo$12bo5b5o$12b3o$16b2o$13b3o2bo$11bob3obo$10bo3bo2bo$11bo4b2ob3o$13b4obo4b2o$13bob4o4b2o$19bo$20bo2b2o$20b2o$21b5o$25b2o$19b3o6bo$20bobo3bobo$19bo3bo3bo$19bo3b2o$18bo6bob3o$19b2o3bo3b2o$20b4o2bo2bo$22b2o3bo$21bo$21b2obo$20bo$19b5o$19bo4bo$18b3ob3o$18bob5o$18bo$20bo$16bo4b4o$20b4ob2o$17b3o4bo$24bobo$28bo$24bo2b2o$25b3o$22b2o$21b3o5bo$24b2o2bobo$21bo2b3obobo$22b2obo2bo$24bobo2b2o$26b2o$22b3o4bo$22b3o4bo$23b2o3b3o$24b2ob2o$25b2o$25bo2$24b2o3b3o$26bo2b2obo$28bo3b2o$29bo2b2o$33bobo$30b2o2b2o$33b2o$33bobo$35b2o$33bob2o$33b3o3bo$32b2o4b2o$32b3o$33bobo$34bobo3b3o$34bo6bob2o$38bo6bo$38bo3b2o$35b2o$39b3o$39b3o$39bo2$39b3o$39b2o!"), 6;
		"Sir Robin with heavyweight minstrel" => life_like_from_rle("4b2o$4bo2bo$4bo3bo$6b3o$2b2o6b4o$2bob2o4b4o$bo4bo6b3o$2b4o4b2o3bo$o9b2o$bo3bo$6b3o2b2o2bo$2b2o7bo4bo$13bob2o$10b2o6bo$11b2ob3obo$10b2o3bo2bo$10bobo2b2o$10bo2bobobo$10b3o6bo$11bobobo3bo$14b2obobo$11bo6b3o2$11bo9bo$11bo3bo6bo$12bo5b5o$12b3o$16b2o$13b3o2bo$11bob3obo$10bo3bo2bo$11bo4b2ob3o$13b4obo4b2o$13bob4o4b2o$19bo$20bo2b2o$20b2o$21b5o$25b2o$19b3o6bo$20bobo3bobo$19bo3bo3bo$19bo3b2o$18bo6bob3o$19b2o3bo3b2o$20b4o2bo2bo$22b2o3bo$21bo$21b2obo$20bo$19b5o$19bo4bo$18b3ob3o$18bob5o$18bo$20bo$16bo4b4o$20b4ob2o$17b3o4bo$24bobo$28bo$24bo2b2o$25b3o$22b2o$21b3o5bo$24b2o2bobo$21bo2b3obobo$22b2obo2bo$24bobo2b2o$26b2o$22b3o4bo$22b3o4bo$23b2o3b3o$24b2ob2o$25b2o$25bo2$24b2o3b3o$26bo2b2obo$28bo3b2o$29bo2b2o$33bobo$30b2o2b2o$33b2o$33bobo$35b2o$33bob2o$33b3o3bo$32b2o4b2o$32b3o$33bobo$34bobo3b3o$34bo6bob2o$38bo6bo$38bo3b2o$35b2o$39b3o$39b3o$39bo2$39b3o$39b3o$39b3o$39b2o!"), 6;
		"Sir Robin with heavyweight-errant minstrel" => life_like_from_rle("4b2o$4bo2bo$4bo3bo$6b3o$2b2o6b4o$2bob2o4b4o$bo4bo6b3o$2b4o4b2o3bo$o9b2o$bo3bo$6b3o2b2o2bo$2b2o7bo4bo$13bob2o$10b2o6bo$11b2ob3obo$10b2o3bo2bo$10bobo2b2o$10bo2bobobo$10b3o6bo$11bobobo3bo$14b2obobo$11bo6b3o2$11bo9bo$11bo3bo6bo$12bo5b5o$12b3o$16b2o$13b3o2bo$11bob3obo$10bo3bo2bo$11bo4b2ob3o$13b4obo4b2o$13bob4o4b2o$19bo$20bo2b2o$20b2o$21b5o$25b2o$19b3o6bo$20bobo3bobo$19bo3bo3bo$19bo3b2o$18bo6bob3o$19b2o3bo3b2o$20b4o2bo2bo$22b2o3bo$21bo$21b2obo$20bo$19b5o$19bo4bo$18b3ob3o$18bob5o$18bo$20bo$16bo4b4o$20b4ob2o$17b3o4bo$24bobo$28bo$24bo2b2o$25b3o$22b2o$21b3o5bo$24b2o2bobo$21bo2b3obobo$22b2obo2bo$24bobo2b2o$26b2o$22b3o4bo$22b3o4bo$23b2o3b3o$24b2ob2o$25b2o$25bo2$24b2o3b3o$26bo2b2obo$28bo3b2o$29bo2b2o$33bobo$30b2o2b2o$33b2o$33bobo$35b2o$33bob2o$33b3o3bo$32b2o4b2o$32b3o$33bobo$34bobo3b3o$34bo6bob2o$38bo6bo$38bo3b2o$35b2o$39b3o$39b3o$39bo2$39b3o$39b3o$39b3o$39b2o3bo$43b3o$43b2o2bo$44bo3bo$45bobo$44bo2bo2b3o$43bo2b2o$42bo4b5o$42bo2bob3o$43bo$45bobo$46b2o$47bo$46b3o2$44b2o$47b3o$43bo2bo$43bo2bo$43bo5bo$42b2ob2obobo$42b2ob2o3bo$43b2obo$44bob2obo!"), 6;
		"Sir Robin with heavyweight-wandering minstrel" => life_like_from_rle("4b2o$4bo2bo$4bo3bo$6b3o$2b2o6b4o$2bob2o4b4o$bo4bo6b3o$2b4o4b2o3bo$o9b2o$bo3bo$6b3o2b2o2bo$2b2o7bo4bo$13bob2o$10b2o6bo$11b2ob3obo$10b2o3bo2bo$10bobo2b2o$10bo2bobobo$10b3o6bo$11bobobo3bo$14b2obobo$11bo6b3o2$11bo9bo$11bo3bo6bo$12bo5b5o$12b3o$16b2o$13b3o2bo$11bob3obo$10bo3bo2bo$11bo4b2ob3o$13b4obo4b2o$13bob4o4b2o$19bo$20bo2b2o$20b2o$21b5o$25b2o$19b3o6bo$20bobo3bobo$19bo3bo3bo$19bo3b2o$18bo6bob3o$19b2o3bo3b2o$20b4o2bo2bo$22b2o3bo$21bo$21b2obo$20bo$19b5o$19bo4bo$18b3ob3o$18bob5o$18bo$20bo$16bo4b4o$20b4ob2o$17b3o4bo$24bobo$28bo$24bo2b2o$25b3o$22b2o$21b3o5bo$24b2o2bobo$21bo2b3obobo$22b2obo2bo$24bobo2b2o$26b2o$22b3o4bo$22b3o4bo$23b2o3b3o$24b2ob2o$25b2o$25bo2$24b2o3b3o$26bo2b2obo$28bo3b2o$29bo2b2o$33bobo$30b2o2b2o$33b2o$33bobo$35b2o$33bob2o$33b3o3bo$32b2o4b2o$32b3o$33bobo$34bobo3b3o$34bo6bob2o$38bo6bo$38bo3b2o$35b2o$39b3o$39b3o$39bo2$39b3o$39b3o$39b3o$39b2o3b2o$43b2o$44b2o$45b2ob2o$46bob2o$45bo3b2o$45bobo3bo2$46b2o$50bo$50bo!"), 6;
		"Sir Robin with lightweight-ragged minstrel" => life_like_from_rle("4b2o$4bo2bo$4bo3bo$6b3o$2b2o6b4o$2bob2o4b4o$bo4bo6b3o$2b4o4b2o3bo$o9b2o$bo3bo$6b3o2b2o2bo$2b2o7bo4bo$13bob2o$10b2o6bo$11b2ob3obo$10b2o3bo2bo$10bobo2b2o$10bo2bobobo$10b3o6bo$11bobobo3bo$14b2obobo$11bo6b3o2$11bo9bo$11bo3bo6bo$12bo5b5o$12b3o$16b2o$13b3o2bo$11bob3obo$10bo3bo2bo$11bo4b2ob3o$13b4obo4b2o$13bob4o4b2o$19bo$20bo2b2o$20b2o$21b5o$25b2o$19b3o6bo$20bobo3bobo$19bo3bo3bo$19bo3b2o$18bo6bob3o$19b2o3bo3b2o$20b4o2bo2bo$22b2o3bo$21bo$21b2obo$20bo$19b5o$19bo4bo$18b3ob3o$18bob5o$18bo$20bo$16bo4b4o$20b4ob2o$17b3o4bo$24bobo$28bo$24bo2b2o$25b3o$22b2o$21b3o5bo$24b2o2bobo$21bo2b3obobo$22b2obo2bo$24bobo2b2o$26b2o$22b3o4bo$22b3o4bo$23b2o3b3o$24b2ob2o$25b2o$25bo2$24b2o3b3o$26bo2b2obo$28bo3b2o$29bo2b2o$33bobo$30b2o2b2o$33b2o$33bobo$35b2o$33bob2o$33b3o3bo$32b2o4b2o$32b3o$33bobo$34bobo3b3o$34bo6bob2o$38bo6bo$38bo3b2o$35b2o$39b3o$39b3o$39bo2$39b3o$39b2o2b3o$43b3o$43bo$46b3o$46b3o$46bo2bo$42b3o2bo2b2o$46bobo$44bo$49bo$43bobo3bo$42b7o$41bo2b2o$44b2o$41b2ob2o$42bo2bo$43bo2bo$43b7o$43bob3obo$44b2o$45b5o$46b4o!"), 6;
		"Sir Robin with lightweight-ragged-errant minstrel" => life_like_from_rle("4b2o$4bo2bo$4bo3bo$6b3o$2b2o6b4o$2bob2o4b4o$bo4bo6b3o$2b4o4b2o3bo$o9b2o$bo3bo$6b3o2b2o2bo$2b2o7bo4bo$13bob2o$10b2o6bo$11b2ob3obo$10b2o3bo2bo$10bobo2b2o$10bo2bobobo$10b3o6bo$11bobobo3bo$14b2obobo$11bo6b3o2$11bo9bo$11bo3bo6bo$12bo5b5o$12b3o$16b2o$13b3o2bo$11bob3obo$10bo3bo2bo$11bo4b2ob3o$13b4obo4b2o$13bob4o4b2o$19bo$20bo2b2o$20b2o$21b5o$25b2o$19b3o6bo$20bobo3bobo$19bo3bo3bo$19bo3b2o$18bo6bob3o$19b2o3bo3b2o$20b4o2bo2bo$22b2o3bo$21bo$21b2obo$20bo$19b5o$19bo4bo$18b3ob3o$18bob5o$18bo$20bo$16bo4b4o$20b4ob2o$17b3o4bo$24bobo$28bo$24bo2b2o$25b3o$22b2o$21b3o5bo$24b2o2bobo$21bo2b3obobo$22b2obo2bo$24bobo2b2o$26b2o$22b3o4bo$22b3o4bo$23b2o3b3o$24b2ob2o$25b2o$25bo2$24b2o3b3o$26bo2b2obo$28bo3b2o$29bo2b2o$33bobo$30b2o2b2o$33b2o$33bobo$35b2o$33bob2o$33b3o3bo$32b2o4b2o$32b3o$33bobo$34bobo3b3o$34bo6bob2o$38bo6bo$38bo3b2o$35b2o$39b3o$39b3o$39bo2$39b3o$39b2o2b3o$43b3o$43bo$46b3o$46b3o$46bo2bo$42b3o2bo2b2o$46bobo$44bo$49bo$43bobo3bo$42b7o$41bo2b2o$44b2o$41b2ob2o$42bo2bo$43bo2bo$43b7o$43bob3obo$44b2o$45b5o$46b4o3bo$53bobo$53bo$54b3o$55bo$58b2o$53bo6b2o$52bob3o3bobo$51bo3b2obobobo$52bob2o$54b2o$54b2o$57bo2$55b2o$54bob2o$54bo3bo$54bo4bo$53bo$53b2obo$52bobo3b2o$52bo4bobo$53bo3b2o$53bo2bo$55bo!"), 6;
		"Sir Robin with lightweight-ragged-wandering minstrel" => life_like_from_rle("4b2o$4bo2bo$4bo3bo$6b3o$2b2o6b4o$2bob2o4b4o$bo4bo6b3o$2b4o4b2o3bo$o9b2o$bo3bo$6b3o2b2o2bo$2b2o7bo4bo$13bob2o$10b2o6bo$11b2ob3obo$10b2o3bo2bo$10bobo2b2o$10bo2bobobo$10b3o6bo$11bobobo3bo$14b2obobo$11bo6b3o2$11bo9bo$11bo3bo6bo$12bo5b5o$12b3o$16b2o$13b3o2bo$11bob3obo$10bo3bo2bo$11bo4b2ob3o$13b4obo4b2o$13bob4o4b2o$19bo$20bo2b2o$20b2o$21b5o$25b2o$19b3o6bo$20bobo3bobo$19bo3bo3bo$19bo3b2o$18bo6bob3o$19b2o3bo3b2o$20b4o2bo2bo$22b2o3bo$21bo$21b2obo$20bo$19b5o$19bo4bo$18b3ob3o$18bob5o$18bo$20bo$16bo4b4o$20b4ob2o$17b3o4bo$24bobo$28bo$24bo2b2o$25b3o$22b2o$21b3o5bo$24b2o2bobo$21bo2b3obobo$22b2obo2bo$24bobo2b2o$26b2o$22b3o4bo$22b3o4bo$23b2o3b3o$24b2ob2o$25b2o$25bo2$24b2o3b3o$26bo2b2obo$28bo3b2o$29bo2b2o$33bobo$30b2o2b2o$33b2o$33bobo$35b2o$33bob2o$33b3o3bo$32b2o4b2o$32b3o$33bobo$34bobo3b3o$34bo6bob2o$38bo6bo$38bo3b2o$35b2o$39b3o$39b3o$39bo2$39b3o$39b2o2b3o$43b3o$43bo$46b3o$46b3o$46bo2bo$42b3o2bo2b2o$46bobo$44bo$49bo$43bobo3bo$42b7o$41bo2b2o$44b2o$41b2ob2o$42bo2bo$43bo2bo$43b7o$43bob3obo$44b2o$45b5o$46b4o3bo$53b2o$54b3o$55b2o$57bo$54bo2b2o$54bo2b2obo$55bo2b2obo$56b2ob2o$56b4o$58bo!"), 6;
		"Sir Robin with lightweight-ragged-featherweight minstrel" => life_like_from_rle("4b2o$4bo2bo$4bo3bo$6b3o$2b2o6b4o$2bob2o4b4o$bo4bo6b3o$2b4o4b2o3bo$o9b2o$bo3bo$6b3o2b2o2bo$2b2o7bo4bo$13bob2o$10b2o6bo$11b2ob3obo$10b2o3bo2bo$10bobo2b2o$10bo2bobobo$10b3o6bo$11bobobo3bo$14b2obobo$11bo6b3o2$11bo9bo$11bo3bo6bo$12bo5b5o$12b3o$16b2o$13b3o2bo$11bob3obo$10bo3bo2bo$11bo4b2ob3o$13b4obo4b2o$13bob4o4b2o$19bo$20bo2b2o$20b2o$21b5o$25b2o$19b3o6bo$20bobo3bobo$19bo3bo3bo$19bo3b2o$18bo6bob3o$19b2o3bo3b2o$20b4o2bo2bo$22b2o3bo$21bo$21b2obo$20bo$19b5o$19bo4bo$18b3ob3o$18bob5o$18bo$20bo$16bo4b4o$20b4ob2o$17b3o4bo$24bobo$28bo$24bo2b2o$25b3o$22b2o$21b3o5bo$24b2o2bobo$21bo2b3obobo$22b2obo2bo$24bobo2b2o$26b2o$22b3o4bo$22b3o4bo$23b2o3b3o$24b2ob2o$25b2o$25bo2$24b2o3b3o$26bo2b2obo$28bo3b2o$29bo2b2o$33bobo$30b2o2b2o$33b2o$33bobo$35b2o$33bob2o$33b3o3bo$32b2o4b2o$32b3o$33bobo$34bobo3b3o$34bo6bob2o$38bo6bo$38bo3b2o$35b2o$39b3o$39b3o$39bo2$39b3o$39b2o2b3o$43b3o$43bo$46b3o$46b3o$46bo2bo$42b3o2bo2b2o$46bobo$44bo$49bo$43bobo3bo$42b7o$41bo2b2o$44b2o$41b2ob2o$42bo2bo$43bo2bo$43b7o$43bob3obo$44b2o$45b5o$46b4o3$48b2obo$49bob2o$46bo2b2o2bo$46bobo4b2o$46bobo3b3o$52bo$53bo!"), 6;
		"Sir Robin with heavyweight-pi minstrel" => life_like_from_rle("4b2o$4bo2bo$4bo3bo$6b3o$2b2o6b4o$2bob2o4b4o$bo4bo6b3o$2b4o4b2o3bo$o9b2o$bo3bo$6b3o2b2o2bo$2b2o7bo4bo$13bob2o$10b2o6bo$11b2ob3obo$10b2o3bo2bo$10bobo2b2o$10bo2bobobo$10b3o6bo$11bobobo3bo$14b2obobo$11bo6b3o2$11bo9bo$11bo3bo6bo$12bo5b5o$12b3o$16b2o$13b3o2bo$11bob3obo$10bo3bo2bo$11bo4b2ob3o$13b4obo4b2o$13bob4o4b2o$19bo$20bo2b2o$20b2o$21b5o$25b2o$19b3o6bo$20bobo3bobo$19bo3bo3bo$19bo3b2o$18bo6bob3o$19b2o3bo3b2o$20b4o2bo2bo$22b2o3bo$21bo$21b2obo$20bo$19b5o$19bo4bo$18b3ob3o$18bob5o$18bo$20bo$16bo4b4o$20b4ob2o$17b3o4bo$24bobo$28bo$24bo2b2o$25b3o$22b2o$21b3o5bo$24b2o2bobo$21bo2b3obobo$22b2obo2bo$24bobo2b2o$26b2o$22b3o4bo$22b3o4bo$23b2o3b3o$24b2ob2o$25b2o$25bo2$24b2o3b3o$26bo2b2obo$28bo3b2o$29bo2b2o$33bobo$30b2o2b2o$33b2o$33bobo$35b2o$33bob2o$33b3o3bo$32b2o4b2o$32b3o$33bobo$34bobo3b3o$34bo6bob2o$38bo6bo$38bo3b2o$35b2o$39b3o$39b3o$39bo2$39b3o$39b3o$39b3o$39b2o3b2o$43b2obo$43b2o$43b2o$45bo$45bob4ob2o$47bo4b2o$44b2obo7bo$49bo4bobo$45b2o3bo3bo$46bobo$50bobo$46bo2b2o2bo$47bo3b2o$48b5o$47bo3bo$46b5o$45bo4bo$45b4obo$44bo4b2o$46b3o$45b3o2bo$50bo2$48bobo$50bo$50bo!"), 6;
		"Sir Robin with heavyweight-wandering-connecting minstrel" => life_like_from_rle("4b2o$4bo2bo$4bo3bo$6b3o$2b2o6b4o$2bob2o4b4o$bo4bo6b3o$2b4o4b2o3bo$o9b2o$bo3bo$6b3o2b2o2bo$2b2o7bo4bo$13bob2o$10b2o6bo$11b2ob3obo$10b2o3bo2bo$10bobo2b2o$10bo2bobobo$10b3o6bo$11bobobo3bo$14b2obobo$11bo6b3o2$11bo9bo$11bo3bo6bo$12bo5b5o$12b3o$16b2o$13b3o2bo$11bob3obo$10bo3bo2bo$11bo4b2ob3o$13b4obo4b2o$13bob4o4b2o$19bo$20bo2b2o$20b2o$21b5o$25b2o$19b3o6bo$20bobo3bobo$19bo3bo3bo$19bo3b2o$18bo6bob3o$19b2o3bo3b2o$20b4o2bo2bo$22b2o3bo$21bo$21b2obo$20bo$19b5o$19bo4bo$18b3ob3o$18bob5o$18bo$20bo$16bo4b4o$20b4ob2o$17b3o4bo$24bobo$28bo$24bo2b2o$25b3o$22b2o$21b3o5bo$24b2o2bobo$21bo2b3obobo$22b2obo2bo$24bobo2b2o$26b2o$22b3o4bo$22b3o4bo$23b2o3b3o$24b2ob2o$25b2o$25bo2$24b2o3b3o$26bo2b2obo$28bo3b2o$29bo2b2o$33bobo$30b2o2b2o$33b2o$33bobo$35b2o$33bob2o$33b3o3bo$32b2o4b2o$32b3o$33bobo$34bobo3b3o$34bo6bob2o$38bo6bo$38bo3b2o$35b2o$39b3o$39b3o$39bo2$39b3o$39b3o$39b3o$39b2o3b2o$43b2o$44b2o$45b2ob2o$46bob2o$45bo3b2o$45bobo3bo2$46b2o$50bo$50bo$51bo$49bobo$48b2obo$49b3o$47b2o$48bobobo$51bo$49bobo$49b2ob2o$51b2obo$52bob2o$54bo$53bo2bo$54bo2bo$57bo2$56bo2bo$56bo3bo$57bobo$56bobo$55bo2bo$55b2ob3o$55b5o2bo$58bo3bo$61bo$61bo$58bobo$59bo2$61bo$60bobo$60bobo$60bobo$60b2o$59bo2bo$58bo2bo$58b2o2bo2bo$58b2ob2o3b2o$59b2obo3b2o$59b2o4bo$64b2o$59b2o2b2o$63bo3bo$64bo2bo$64b3o$63b2o$63b2o2b2o$67bo$64b2obo$66b2obo$66bo2bo$68b3o$65b3o$64bobo$64bo2$65bo2bo$65b3o$68b2o$63b3ob2o$66b3o$62bo3b2o$63bobo!"), 6;
		"Sir Robin with errant-fountain minstrel" => life_like_from_rle("4b2o$4bo2bo$4bo3bo$6b3o$2b2o6b4o$2bob2o4b4o$bo4bo6b3o$2b4o4b2o3bo$o9b2o$bo3bo$6b3o2b2o2bo$2b2o7bo4bo$13bob2o$10b2o6bo$11b2ob3obo$10b2o3bo2bo$10bobo2b2o$10bo2bobobo$10b3o6bo$11bobobo3bo$14b2obobo$11bo6b3o2$11bo9bo$11bo3bo6bo$12bo5b5o$12b3o$16b2o$13b3o2bo$11bob3obo$10bo3bo2bo$11bo4b2ob3o$13b4obo4b2o$13bob4o4b2o$19bo$20bo2b2o$20b2o$21b5o$25b2o$19b3o6bo$20bobo3bobo$19bo3bo3bo$19bo3b2o$18bo6bob3o$19b2o3bo3b2o$20b4o2bo2bo$22b2o3bo$21bo$21b2obo$20bo$19b5o$19bo4bo$18b3ob3o$18bob5o$18bo$20bo$16bo4b4o$20b4ob2o$17b3o4bo$24bobo$28bo$24bo2b2o$25b3o$22b2o$21b3o5bo$24b2o2bobo$21bo2b3obobo$22b2obo2bo$24bobo2b2o$26b2o$22b3o4bo$22b3o4bo$23b2o3b3o$24b2ob2o$25b2o$25bo2$24b2o4b3o$26bo5b2o$33bo$35bo$31bo4bo$37bo$31bo4bo2b2o$30b2ob3o3bo$32bob2o$30bobob2o$31bo3b2o$34b2o$35b2o$34bo$33bo2bo$33bo$32bo3bo$33b2ob2o$34bobo$30b3o3bo$30b2o3b2o$32b2obob2o$34bo$31bo$31bo6bo$36b2obo$37bo2bo$39bo$38b2o$38b2o$37bo2$37b3o$36bo$36bo5bo$35bo6bo$35bo2b3obo$36bobo2bo$38bo$36bo4bobo$36b3o3bobo$38bo3bobo2$39b2o$39b3o$41bobo$45bo$41bo3b2ob3o$45b2o$42bo5b3o$42bo4bo$47bo$44bo4b2o$44b3o4bo$43bo$44b3o3bo$48b3o$44bo3bo$44bob2o$45b3o3b3o2$47b2o2bobo$46bo4b2o$46b2ob2o$46bob2ob2o$50b4o$48bo$47bo2bo$48bo$49b2o2bo2$52b2o$51bo$52b3o$53bo$50b2ob2o$50bo2b3o2$52bo3bo$55b2o$53b2o!"), 6;
		"Sir Robin with lightweight-ragged-pi minstrel" => life_like_from_rle("4b2o$4bo2bo$4bo3bo$6b3o$2b2o6b4o$2bob2o4b4o$bo4bo6b3o$2b4o4b2o3bo$o9b2o$bo3bo$6b3o2b2o2bo$2b2o7bo4bo$13bob2o$10b2o6bo$11b2ob3obo$10b2o3bo2bo$10bobo2b2o$10bo2bobobo$10b3o6bo$11bobobo3bo$14b2obobo$11bo6b3o2$11bo9bo$11bo3bo6bo$12bo5b5o$12b3o$16b2o$13b3o2bo$11bob3obo$10bo3bo2bo$11bo4b2ob3o$13b4obo4b2o$13bob4o4b2o$19bo$20bo2b2o$20b2o$21b5o$25b2o$19b3o6bo$20bobo3bobo$19bo3bo3bo$19bo3b2o$18bo6bob3o$19b2o3bo3b2o$20b4o2bo2bo$22b2o3bo$21bo$21b2obo$20bo$19b5o$19bo4bo$18b3ob3o$18bob5o$18bo$20bo$16bo4b4o$20b4ob2o$17b3o4bo$24bobo$28bo$24bo2b2o$25b3o$22b2o$21b3o5bo$24b2o2bobo$21bo2b3obobo$22b2obo2bo$24bobo2b2o$26b2o$22b3o4bo$22b3o4bo$23b2o3b3o$24b2ob2o$25b2o$25bo2$24b2o3b3o$26bo2b2obo$28bo3b2o$29bo2b2o$33bobo$30b2o2b2o$33b2o$33bobo$35b2o$33bob2o$33b3o3bo$32b2o4b2o$32b3o$33bobo$34bobo3b3o$34bo6bob2o$38bo6bo$38bo3b2o$35b2o$39b3o$39b3o$39bo2$39b3o$39b2o2b3o$43b3o$43bo$46b3o$46b3o$46bo2bo$42b3o2bo2b2o$46bobo$44bo$49bo$43bobo3bo$42b7o$41bo2b2o$44b2o$41b2ob2o$42bo2bo$43bo2bo$43b7o$43bob3obo$44b2o$45b5o$46b4o3b2o$53b2o$55bo$54bo$53b3o$55bo$53b2ob2o2b3o$53b2o3b2o2bo$54bo5b2ob3o$60bo3b2o$55bo$55b2o3b2o$57b2ob2o$59bob2o$57b3o$56bo$57bobo$55bo3bo$55b4o$54bo3bo$53b3o$55bobo$57bobo$56b5o$57bob2o$58b2o!"), 6;
		"Sir Robin with lightweight-ragged-wandering-connecting minstrel" => life_like_from_rle("4b2o$4bo2bo$4bo3bo$6b3o$2b2o6b4o$2bob2o4b4o$bo4bo6b3o$2b4o4b2o3bo$o9b2o$bo3bo$6b3o2b2o2bo$2b2o7bo4bo$13bob2o$10b2o6bo$11b2ob3obo$10b2o3bo2bo$10bobo2b2o$10bo2bobobo$10b3o6bo$11bobobo3bo$14b2obobo$11bo6b3o2$11bo9bo$11bo3bo6bo$12bo5b5o$12b3o$16b2o$13b3o2bo$11bob3obo$10bo3bo2bo$11bo4b2ob3o$13b4obo4b2o$13bob4o4b2o$19bo$20bo2b2o$20b2o$21b5o$25b2o$19b3o6bo$20bobo3bobo$19bo3bo3bo$19bo3b2o$18bo6bob3o$19b2o3bo3b2o$20b4o2bo2bo$22b2o3bo$21bo$21b2obo$20bo$19b5o$19bo4bo$18b3ob3o$18bob5o$18bo$20bo$16bo4b4o$20b4ob2o$17b3o4bo$24bobo$28bo$24bo2b2o$25b3o$22b2o$21b3o5bo$24b2o2bobo$21bo2b3obobo$22b2obo2bo$24bobo2b2o$26b2o$22b3o4bo$22b3o4bo$23b2o3b3o$24b2ob2o$25b2o$25bo2$24b2o3b3o$26bo2b2obo$28bo3b2o$29bo2b2o$33bobo$30b2o2b2o$33b2o$33bobo$35b2o$33bob2o$33b3o3bo$32b2o4b2o$32b3o$33bobo$34bobo3b3o$34bo6bob2o$38bo6bo$38bo3b2o$35b2o$39b3o$39b3o$39bo2$39b3o$39b2o2b3o$43b3o$43bo$46b3o$46b3o$46bo2bo$42b3o2bo2b2o$46bobo$44bo$49bo$43bobo3bo$42b7o$41bo2b2o$44b2o$41b2ob2o$42bo2bo$43bo2bo$43b7o$43bob3obo$44b2o$45b5o$46b4o3bo$53b2o$54b3o$55b2o$57bo$54bo2b2o$54bo2b2obo$55bo2b2obo$56b2ob2o$56b4o$58bo2$59bo$58b3o$57bo2bo$57bob3o$59bo$56b2o2bo$59b2o$59bob2o$61b2o$58bo4bo$60bobo$60bo3bo$62b4o$66bo$64b3o2$67b3o$66bo$66bo2bo$64b2o$64b2obobo$67bo3bo$64b4o3bo$64b2o2b2obo$68bo$68bobo$67bo2$68bo2bo2$69b2obo$69b2obo$70bo$69bo$67bob2o$71bobo$67bo3b5o$68b2o5b2o$69bob3o$69b2ob2ob2o$69b3o2b2o$69b2o4b2o2$73b2o$73b4o$73b4o$77bo$74bo$73bo2b2o$74bob3o$76bo2bo$75bo4bo$74bo$74bo2bo$74bobo$75bo$75bobo$74b2ob2o$74b2ob2o$73bo2bo$72b2o$73bo!"), 6;
		"Sir Robin with heavyweight-errant-fountain minstrel" => life_like_from_rle("4b2o$4bo2bo$4bo3bo$6b3o$2b2o6b4o$2bob2o4b4o$bo4bo6b3o$2b4o4b2o3bo$o9b2o$bo3bo$6b3o2b2o2bo$2b2o7bo4bo$13bob2o$10b2o6bo$11b2ob3obo$10b2o3bo2bo$10bobo2b2o$10bo2bobobo$10b3o6bo$11bobobo3bo$14b2obobo$11bo6b3o2$11bo9bo$11bo3bo6bo$12bo5b5o$12b3o$16b2o$13b3o2bo$11bob3obo$10bo3bo2bo$11bo4b2ob3o$13b4obo4b2o$13bob4o4b2o$19bo$20bo2b2o$20b2o$21b5o$25b2o$19b3o6bo$20bobo3bobo$19bo3bo3bo$19bo3b2o$18bo6bob3o$19b2o3bo3b2o$20b4o2bo2bo$22b2o3bo$21bo$21b2obo$20bo$19b5o$19bo4bo$18b3ob3o$18bob5o$18bo$20bo$16bo4b4o$20b4ob2o$17b3o4bo$24bobo$28bo$24bo2b2o$25b3o$22b2o$21b3o5bo$24b2o2bobo$21bo2b3obobo$22b2obo2bo$24bobo2b2o$26b2o$22b3o4bo$22b3o4bo$23b2o3b3o$24b2ob2o$25b2o$25bo2$24b2o3b3o$26bo2b2obo$28bo3b2o$29bo2b2o$33bobo$30b2o2b2o$33b2o$33bobo$35b2o$33bob2o$33b3o3bo$32b2o4b2o$32b3o$33bobo$34bobo3b3o$34bo6bob2o$38bo6bo$38bo3b2o$35b2o$39b3o$39b3o$39bo2$39b3o$39b3o$39b3o$39b2o3bo$43b3o$43b2o2bo$44bo3bo$45bobo$44bo2bo2b3o$43bo2b2o$42bo4b5o$42bo2bob3o$43bo$45bobo$46b2o$47bo$46b3o2$44b2o$47b3o$43bo2bo$43bo2bo$43bo5bo$42b2ob2obobo$42b2ob2o3bo$43b2obo$44bob2o$49b2o$50bobo2$50bo2bo2$50b2o$50bobo2$48b2obo2$47bo2bob3o$48b2o2bo2bo$52bo$49b5o$49bo2b2obo$49bo4bobo$50bo5bo$52bo2bo$51bo2bo$53b3o$52bob2o$52bob2o2bo$58b3o$54bo2bobob2o$54b3obo3bo$57b2o2$58bo5bo$59b2o$55b3o3b3o2$58bobo$55b2obo3bo$57b2o3bo$60bob2o$61b2ob2o$61b2o2bo$61bo$58b2o2bo2bo$57b2o3bo2bo$57b2o5b3o$58b3obo2b2o$59bo$59bo2b3o$60bo2bo2$64b2o$64b2o$63b2obo2$63b2o2b2o$63bobo$63bo2b2o$67bo$64b2obo$66bo!"), 6;
		"Sir Robin with lightweight-ragged-errant-fountain minstrel" => life_like_from_rle("4b2o$4bo2bo$4bo3bo$6b3o$2b2o6b4o$2bob2o4b4o$bo4bo6b3o$2b4o4b2o3bo$o9b2o$bo3bo$6b3o2b2o2bo$2b2o7bo4bo$13bob2o$10b2o6bo$11b2ob3obo$10b2o3bo2bo$10bobo2b2o$10bo2bobobo$10b3o6bo$11bobobo3bo$14b2obobo$11bo6b3o2$11bo9bo$11bo3bo6bo$12bo5b5o$12b3o$16b2o$13b3o2bo$11bob3obo$10bo3bo2bo$11bo4b2ob3o$13b4obo4b2o$13bob4o4b2o$19bo$20bo2b2o$20b2o$21b5o$25b2o$19b3o6bo$20bobo3bobo$19bo3bo3bo$19bo3b2o$18bo6bob3o$19b2o3bo3b2o$20b4o2bo2bo$22b2o3bo$21bo$21b2obo$20bo$19b5o$19bo4bo$18b3ob3o$18bob5o$18bo$20bo$16bo4b4o$20b4ob2o$17b3o4bo$24bobo$28bo$24bo2b2o$25b3o$22b2o$21b3o5bo$24b2o2bobo$21bo2b3obobo$22b2obo2bo$24bobo2b2o$26b2o$22b3o4bo$22b3o4bo$23b2o3b3o$24b2ob2o$25b2o$25bo2$24b2o3b3o$26bo2b2obo$28bo3b2o$29bo2b2o$33bobo$30b2o2b2o$33b2o$33bobo$35b2o$33bob2o$33b3o3bo$32b2o4b2o$32b3o$33bobo$34bobo3b3o$34bo6bob2o$38bo6bo$38bo3b2o$35b2o$39b3o$39b3o$39bo2$39b3o$39b2o2b3o$43b3o$43bo$46b3o$46b3o$46bo2bo$42b3o2bo2b2o$46bobo$44bo$49bo$43bobo3bo$42b7o$41bo2b2o$44b2o$41b2ob2o$42bo2bo$43bo2bo$43b7o$43bob3obo$44b2o$45b5o$46b4o3bo$53bobo$53bo$54b3o$55bo$58b2o$53bo6b2o$52bob3o3bobo$51bo3b2obobobo$52bob2o$54b2o$54b2o$57bo2$55b2o$54bob2o$54bo3bo$54bo4bo$53bo$53b2obo$52bobo3b2o$52bo4bobo$53bo3b2o$53bo2bo$55bo$58b3o$61b2o$58b2o$60bo2bo$60bobo$60b2o$59b2o$59bo$58bobo$57b2obo2$56bo4b5o$57bo6b2o$58bobobobo$59bobo2b2o2$60b2ob2o$59b3o$60b3o$60b2ob2o2$62bo4bo$63b2ob3obo$65b2o2b2o$67bo3bo$67bo3bo$68bo2bo$69b3o$65b2o4bo$65bobo$65bo$65b3o3bo$66bob3o$66b2o2bo$66b5o$69b2o2b3o$68b3o2bo$69b2o2bo$67b4o2b2o$67b2ob3obo$67b2obo$69b2ob4o$72b2o$69bo3bo$69bo2bo$71bo$74b2o$73b2o$73bo2bo$73b4o$72b2ob2o$76bo$72bo2bob2o$73bobo2$75bo$75bo2bo!"), 6;*/
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
	print_von_neumann_rule_tree!(
		rule!{Langton's Ant "LLR"},
		"ABCA^A>AvA<B^B>BvB<C^C>CvC<"
	);
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
}
/*
function f(str) {
	let cells = str.replace(/[\r\n ]/g,"").replace("!","$").match(/\d*[bo$]/g)
	cells = cells.map(s => s.length<=1 ? s : s.substr(-1).repeat(parseInt(s.substr(0, s.length-1))))
	cells = cells.join("")
	let pattern = []
	let temp = ""
	for (let c of cells) {
		if (c == "$") {
			pattern.push(temp)
			temp = ""
		} else if (c == "b") {
			temp += "."
		} else if (c == "o") {
			temp += "#"
		}
	}
	let patLens = pattern.map(l => l.length)
	let maxLen = Math.max.apply(null, patLens)
	for (let i in pattern) {
		if (pattern[i].length < maxLen)
			pattern[i] += ".".repeat(maxLen - pattern[i].length)
	}
	return pattern
}
*/