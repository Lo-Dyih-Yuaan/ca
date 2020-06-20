//$D_4$群对称
macro_rules! d4_symmetry {
	(@ ;
	 $p11:pat,$p12:pat,$p13:pat,$p21:pat,$p23:pat,$p31:pat,$p32:pat,$p33:pat) => 
		{($p11,$p12,$p13,$p21,$p23,$p31,$p32,$p33)};
	(@ ;
		$p1:pat,$p2:pat,$p3:pat,$p4:pat) => 
		   {($p1,$p2,$p3,$p4)};
	(@ rotate $($i:ident)*; //旋转（向右90°）
	 $p11:pat, $p12:pat, $p13:pat,
	 $p21:pat,           $p23:pat,
	 $p31:pat, $p32:pat, $p33:pat) => {
		d4_symmetry!(@ $($i)*;
			$p31, $p21, $p11,
			$p32,       $p12,
			$p33, $p23, $p13)
	};
	(@ rotate $($i:ident)*; //旋转（向右90°）
	         $pn:pat,
	 $pw:pat,         $pe:pat,
	         $ps:pat) => {
		d4_symmetry!(@ $($i)*;
			    $pw,
			$ps,    $pn,
			    $pe)
	};
	(@ flip $($i:ident)*; //翻转（垂直）
	 $p11:pat, $p12:pat, $p13:pat,
	 $p21:pat,           $p23:pat,
	 $p31:pat, $p32:pat, $p33:pat) => {
		d4_symmetry!(@ $($i)*;
			$p31, $p32, $p33,
			$p21,       $p23,
			$p11, $p12, $p13)
	};
	(@ flip $($i:ident)*; //翻转（垂直）
	         $pn:pat,
	 $pw:pat,         $pe:pat,
	         $ps:pat) => {
		d4_symmetry!(@ $($i)*;
			    $ps,
			$pw,    $pe,
			    $pn)
	};
	($t:ty;$p11:pat,$p12:pat,$p13:pat,$p21:pat,$p23:pat,$p31:pat,$p32:pat,$p33:pat) => {
		Box::new(
			|c11:&$t,c12:&$t,c13:&$t,c21:&$t,c23:&$t,c31:&$t,c32:&$t,c33:&$t|
			d4_symmetry!{
				$p11,$p12,$p13,$p21,$p23,$p31,$p32,$p33,
				(c11,c12,c13,c21,c23,c31,c32,c33)
			}
		) as Box<dyn Fn(&$t,&$t,&$t,&$t,&$t,&$t,&$t,&$t) -> bool>
	};
	($p11:pat,$p12:pat,$p13:pat,$p21:pat,$p23:pat,$p31:pat,$p32:pat,$p33:pat, $x:expr) => {
		matches!{
			$x,
			d4_symmetry!(@; $p11,$p12,$p13,$p21,$p23,$p31,$p32,$p33) |
			d4_symmetry!(@rotate; $p11,$p12,$p13,$p21,$p23,$p31,$p32,$p33) |
			d4_symmetry!(@rotate rotate; $p11,$p12,$p13,$p21,$p23,$p31,$p32,$p33) |
			d4_symmetry!(@rotate rotate rotate; $p11,$p12,$p13,$p21,$p23,$p31,$p32,$p33) |
			d4_symmetry!(@flip; $p11,$p12,$p13,$p21,$p23,$p31,$p32,$p33) |
			d4_symmetry!(@flip rotate; $p11,$p12,$p13,$p21,$p23,$p31,$p32,$p33) |
			d4_symmetry!(@flip rotate rotate; $p11,$p12,$p13,$p21,$p23,$p31,$p32,$p33) |
			d4_symmetry!(@flip rotate rotate rotate; $p11,$p12,$p13,$p21,$p23,$p31,$p32,$p33)
		}
	};
	($p1:pat,$p2:pat,$p3:pat,$p4:pat, $x:expr) => {
		matches!{
			$x,
			d4_symmetry!(@; $p1,$p2,$p3,$p4) |
			d4_symmetry!(@rotate; $p1,$p2,$p3,$p4) |
			d4_symmetry!(@rotate rotate; $p1,$p2,$p3,$p4) |
			d4_symmetry!(@rotate rotate rotate; $p1,$p2,$p3,$p4) |
			d4_symmetry!(@flip; $p1,$p2,$p3,$p4) |
			d4_symmetry!(@flip rotate; $p1,$p2,$p3,$p4) |
			d4_symmetry!(@flip rotate rotate; $p1,$p2,$p3,$p4) |
			d4_symmetry!(@flip rotate rotate rotate; $p1,$p2,$p3,$p4)
		}
	};
}

/*使用该宏可能引起大量警告，建议使用`#[allow(unreachable_patterns)]`阻止*/
macro_rules! non_totalistic_closure {
	/*宏内部使用*/
	($t:ty; $p:pat, $n:expr, $c:expr) => {
		match ($n, $c) {
			//1
			(1, 'c') => d4_symmetry!($t;$p,_,_,_,_,_,_,_),
			(1, 'e') => d4_symmetry!($t;_,$p,_,_,_,_,_,_),
			//2
			(2, 'c') => d4_symmetry!($t;$p,_,$p,_,_,_,_,_),
			(2, 'e') => d4_symmetry!($t;_,$p,_,$p,_,_,_,_),
			(2, 'k') => d4_symmetry!($t;$p,_,_,_,$p,_,_,_),
			(2, 'a') => d4_symmetry!($t;$p,$p,_,_,_,_,_,_),
			(2, 'i') => d4_symmetry!($t;_,$p,_,_,_,_,$p,_),
			(2, 'n') => d4_symmetry!($t;$p,_,_,_,_,_,_,$p),
			//3
			(3, 'c') => d4_symmetry!($t;$p,_,$p,_,_,$p,_,_),
			(3, 'e') => d4_symmetry!($t;_,$p,_,$p,$p,_,_,_),
			(3, 'k') => d4_symmetry!($t;$p,_,_,_,$p,_,$p,_),
			(3, 'a') => d4_symmetry!($t;$p,$p,_,$p,_,_,_,_),
			(3, 'i') => d4_symmetry!($t;$p,$p,$p,_,_,_,_,_),
			(3, 'n') => d4_symmetry!($t;$p,_,$p,_,$p,_,_,_),
			(3, 'y') => d4_symmetry!($t;$p,_,$p,_,_,_,$p,_),
			(3, 'q') => d4_symmetry!($t;$p,$p,_,_,_,_,_,$p),
			(3, 'j') => d4_symmetry!($t;$p,$p,_,_,$p,_,_,_),
			(3, 'r') => d4_symmetry!($t;$p,$p,_,_,_,_,$p,_),
			//4
			(4, 'c') => d4_symmetry!($t;$p,_,$p,_,_,$p,_,$p),
			(4, 'e') => d4_symmetry!($t;_,$p,_,$p,$p,_,$p,_),
			(4, 'k') => d4_symmetry!($t;$p,_,$p,$p,_,_,$p,_),
			(4, 'a') => d4_symmetry!($t;$p,$p,$p,_,$p,_,_,_),
			(4, 'i') => d4_symmetry!($t;$p,_,$p,$p,$p,_,_,_),
			(4, 'n') => d4_symmetry!($t;$p,$p,$p,_,_,$p,_,_),
			(4, 'y') => d4_symmetry!($t;$p,_,$p,_,$p,$p,_,_),
			(4, 'q') => d4_symmetry!($t;$p,$p,_,$p,_,_,_,$p),
			(4, 'j') => d4_symmetry!($t;$p,$p,_,_,$p,_,$p,_),
			(4, 'r') => d4_symmetry!($t;$p,$p,_,$p,$p,_,_,_),
			(4, 't') => d4_symmetry!($t;$p,$p,$p,_,_,_,$p,_),
			(4, 'w') => d4_symmetry!($t;$p,$p,_,_,$p,_,_,$p),
			(4, 'z') => d4_symmetry!($t;$p,$p,_,_,_,_,$p,$p),
			//5
			(5, 'c') => d4_symmetry!($t;_,$p,_,$p,$p,_,$p,$p),
			(5, 'e') => d4_symmetry!($t;$p,_,$p,_,_,$p,$p,$p),
			(5, 'k') => d4_symmetry!($t;_,$p,$p,$p,_,$p,_,$p),
			(5, 'a') => d4_symmetry!($t;_,_,$p,_,$p,$p,$p,$p),
			(5, 'i') => d4_symmetry!($t;_,_,_,$p,$p,$p,$p,$p),
			(5, 'n') => d4_symmetry!($t;_,$p,_,$p,_,$p,$p,$p),
			(5, 'y') => d4_symmetry!($t;_,$p,_,$p,$p,$p,_,$p),
			(5, 'q') => d4_symmetry!($t;_,_,$p,$p,$p,$p,$p,_),
			(5, 'j') => d4_symmetry!($t;_,_,$p,$p,_,$p,$p,$p),
			(5, 'r') => d4_symmetry!($t;_,_,$p,$p,$p,$p,_,$p),
			//6
			(6, 'c') => d4_symmetry!($t;_,$p,_,$p,$p,$p,$p,$p),
			(6, 'e') => d4_symmetry!($t;$p,_,$p,_,$p,$p,$p,$p),
			(6, 'k') => d4_symmetry!($t;_,$p,$p,$p,_,$p,$p,$p),
			(6, 'a') => d4_symmetry!($t;_,_,$p,$p,$p,$p,$p,$p),
			(6, 'i') => d4_symmetry!($t;$p,_,$p,$p,$p,$p,_,$p),
			(6, 'n') => d4_symmetry!($t;_,$p,$p,$p,$p,$p,$p,_),
			//7
			(7, 'c') => d4_symmetry!($t;_,$p,$p,$p,$p,$p,$p,$p),
			(7, 'e') => d4_symmetry!($t;$p,_,$p,$p,$p,$p,$p,$p),
			//其他
			_ => unreachable!()
		}
	};
	($t:ty; $p:pat, $s:expr) => {{
		let i_n = $s.matches(char::is_numeric)
			.map(|s|<usize as std::str::FromStr>::from_str(s).ok().unwrap());
		let i_str = {
			let mut temp = $s.split(char::is_numeric);
			temp.next();
			temp
		};
		let i = i_n.zip(i_str).map(|(n, s)| {
			let is_inversed = s.starts_with("-");
			let s = if is_inversed {&s[1..]} else {s};
			let fs = s.chars().map(move|c|non_totalistic_closure!($t; $p, n, c));
			let fs: Vec<_> = fs.collect();
			(n, is_inversed, fs)
		});
		let v: Vec<(usize, bool, Vec<_>)> = i.collect();
		move |c11:&$t,c12:&$t,c13:&$t,c21:&$t,c23:&$t,c31:&$t,c32:&$t,c33:&$t| {
			let mut sum: usize = 0;
			if let $p = c11 {sum+=1;}
			if let $p = c12 {sum+=1;}
			if let $p = c13 {sum+=1;}
			if let $p = c21 {sum+=1;}
			if let $p = c23 {sum+=1;}
			if let $p = c31 {sum+=1;}
			if let $p = c32 {sum+=1;}
			if let $p = c33 {sum+=1;}
			for (n, is_inversed, fs) in &v {
				if *n == sum {
					return fs.is_empty() ||
						(is_inversed ^ fs.iter().any(|f| f(c11,c12,c13,c21,c23,c31,c32,c33)));
				}
			}
			return false;
		}
	}};
}
