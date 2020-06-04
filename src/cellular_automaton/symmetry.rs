macro_rules! non_totalistic {
	(@ ;
	 $p11:pat,$p12:pat,$p13:pat,$p21:pat,$p23:pat,$p31:pat,$p32:pat,$p33:pat) => 
		{($p11,$p12,$p13,$p21,$p23,$p31,$p32,$p33)};
	(@ rotate $($i:ident)*; //旋转（向右90°）
	 $p11:pat, $p12:pat, $p13:pat,
	 $p21:pat,           $p23:pat,
	 $p31:pat, $p32:pat, $p33:pat) => {
		non_totalistic!(@ $($i)*;
			$p31, $p21, $p11,
			$p32,       $p12,
			$p33, $p23, $p13)
	};
	(@ flip $($i:ident)*; //翻转（垂直）
	 $p11:pat, $p12:pat, $p13:pat,
	 $p21:pat,           $p23:pat,
	 $p31:pat, $p32:pat, $p33:pat) => {
		non_totalistic!(@ $($i)*;
			$p31, $p32, $p33,
			$p21,       $p23,
			$p11, $p12, $p13)
	};
	($p11:pat,$p12:pat,$p13:pat,$p21:pat,$p23:pat,$p31:pat,$p32:pat,$p33:pat, $x:expr) => {
		matches!{
			$x,
			non_totalistic!(@; $p11,$p12,$p13,$p21,$p23,$p31,$p32,$p33) |
			non_totalistic!(@rotate; $p11,$p12,$p13,$p21,$p23,$p31,$p32,$p33) |
			non_totalistic!(@rotate rotate; $p11,$p12,$p13,$p21,$p23,$p31,$p32,$p33) |
			non_totalistic!(@rotate rotate rotate; $p11,$p12,$p13,$p21,$p23,$p31,$p32,$p33) |
			non_totalistic!(@flip; $p11,$p12,$p13,$p21,$p23,$p31,$p32,$p33) |
			non_totalistic!(@flip rotate; $p11,$p12,$p13,$p21,$p23,$p31,$p32,$p33) |
			non_totalistic!(@flip rotate rotate; $p11,$p12,$p13,$p21,$p23,$p31,$p32,$p33) |
			non_totalistic!(@flip rotate rotate rotate; $p11,$p12,$p13,$p21,$p23,$p31,$p32,$p33)
		}
	}
}

/*macro_rules! non_totalistic_v {
	(@ ;
	 $p1:pat,$p2:pat,$p3:pat,$p4:pat) => 
		{($p1,$p2,$p3,$p4)};
	(@ rotate $($i:ident)*; //旋转（向右90°）
	         $pn:pat,
	 $pw:pat,         $pe:pat,
	         $ps:pat) => {
		non_totalistic_v!(@ $($i)*;
			    $pw,
			$ps,    $pn,
			    $pe)
	};
	(@ flip $($i:ident)*; //翻转（垂直）
	         $pn:pat,
	 $pw:pat,         $pe:pat,
	         $ps:pat) => {
		non_totalistic_v!(@ $($i)*;
			    $ps,
			$pw,    $pe,
			    $pn)
	};
	($p1:pat,$p2:pat,$p3:pat,$p4:pat , $x:expr) => {
		matches!{
			$x,
			non_totalistic_v!(@; $p1,$p2,$p3,$p4) |
			non_totalistic_v!(@rotate; $p1,$p2,$p3,$p4) |
			non_totalistic_v!(@rotate rotate; $p1,$p2,$p3,$p4) |
			non_totalistic_v!(@rotate rotate rotate; $p1,$p2,$p3,$p4) |
			non_totalistic_v!(@flip; $p1,$p2,$p3,$p4) |
			non_totalistic_v!(@flip rotate; $p1,$p2,$p3,$p4) |
			non_totalistic_v!(@flip rotate rotate; $p1,$p2,$p3,$p4) |
			non_totalistic_v!(@flip rotate rotate rotate; $p1,$p2,$p3,$p4)
		}
	}
}*/

/*使用该宏可能引起大量警告，建议使用`#[allow(unreachable_patterns)]`阻止*/
macro_rules! non_totalistic_closure {
	/*宏内部使用*/
	($t:ty;$p11:pat,$p12:pat,$p13:pat,$p21:pat,$p23:pat,$p31:pat,$p32:pat,$p33:pat) => {
		Box::new(
			|c11:&$t,c12:&$t,c13:&$t,c21:&$t,c23:&$t,c31:&$t,c32:&$t,c33:&$t|
			non_totalistic!{
				$p11,$p12,$p13,$p21,$p23,$p31,$p32,$p33,
				(c11,c12,c13,c21,c23,c31,c32,c33)
			}
		) as Box<dyn Fn(&$t,&$t,&$t,&$t,&$t,&$t,&$t,&$t) -> bool>
	};
	/*宏内部使用*/
	($t:ty; $p:pat, $n:expr, $c:expr) => {
		match ($n, $c) {
			//1
			(1, 'c') => non_totalistic_closure!($t;$p,_,_,_,_,_,_,_),
			(1, 'e') => non_totalistic_closure!($t;_,$p,_,_,_,_,_,_),
			//2
			(2, 'c') => non_totalistic_closure!($t;$p,_,$p,_,_,_,_,_),
			(2, 'e') => non_totalistic_closure!($t;_,$p,_,$p,_,_,_,_),
			(2, 'k') => non_totalistic_closure!($t;$p,_,_,_,$p,_,_,_),
			(2, 'a') => non_totalistic_closure!($t;$p,$p,_,_,_,_,_,_),
			(2, 'i') => non_totalistic_closure!($t;_,$p,_,_,_,_,$p,_),
			(2, 'n') => non_totalistic_closure!($t;$p,_,_,_,_,_,_,$p),
			//3
			(3, 'c') => non_totalistic_closure!($t;$p,_,$p,_,_,$p,_,_),
			(3, 'e') => non_totalistic_closure!($t;_,$p,_,$p,$p,_,_,_),
			(3, 'k') => non_totalistic_closure!($t;$p,_,_,_,$p,_,$p,_),
			(3, 'a') => non_totalistic_closure!($t;$p,$p,_,$p,_,_,_,_),
			(3, 'i') => non_totalistic_closure!($t;$p,$p,$p,_,_,_,_,_),
			(3, 'n') => non_totalistic_closure!($t;$p,_,$p,_,$p,_,_,_),
			(3, 'y') => non_totalistic_closure!($t;$p,_,$p,_,_,_,$p,_),
			(3, 'q') => non_totalistic_closure!($t;$p,$p,_,_,_,_,_,$p),
			(3, 'j') => non_totalistic_closure!($t;$p,$p,_,_,$p,_,_,_),
			(3, 'r') => non_totalistic_closure!($t;$p,$p,_,_,_,_,$p,_),
			//4
			(4, 'c') => non_totalistic_closure!($t;$p,_,$p,_,_,$p,_,$p),
			(4, 'e') => non_totalistic_closure!($t;_,$p,_,$p,$p,_,$p,_),
			(4, 'k') => non_totalistic_closure!($t;$p,_,$p,$p,_,_,$p,_),
			(4, 'a') => non_totalistic_closure!($t;$p,$p,$p,_,$p,_,_,_),
			(4, 'i') => non_totalistic_closure!($t;$p,_,$p,$p,$p,_,_,_),
			(4, 'n') => non_totalistic_closure!($t;$p,$p,$p,_,_,$p,_,_),
			(4, 'y') => non_totalistic_closure!($t;$p,_,$p,_,$p,$p,_,_),
			(4, 'q') => non_totalistic_closure!($t;$p,$p,_,$p,_,_,_,$p),
			(4, 'j') => non_totalistic_closure!($t;$p,$p,_,_,$p,_,$p,_),
			(4, 'r') => non_totalistic_closure!($t;$p,$p,_,$p,$p,_,_,_),
			(4, 't') => non_totalistic_closure!($t;$p,$p,$p,_,_,_,$p,_),
			(4, 'w') => non_totalistic_closure!($t;$p,$p,_,_,$p,_,_,$p),
			(4, 'z') => non_totalistic_closure!($t;$p,$p,_,_,_,_,$p,$p),
			//5
			(5, 'c') => non_totalistic_closure!($t;_,$p,_,$p,$p,_,$p,$p),
			(5, 'e') => non_totalistic_closure!($t;$p,_,$p,_,_,$p,$p,$p),
			(5, 'k') => non_totalistic_closure!($t;_,$p,$p,$p,_,$p,_,$p),
			(5, 'a') => non_totalistic_closure!($t;_,_,$p,_,$p,$p,$p,$p),
			(5, 'i') => non_totalistic_closure!($t;_,_,_,$p,$p,$p,$p,$p),
			(5, 'n') => non_totalistic_closure!($t;_,$p,_,$p,_,$p,$p,$p),
			(5, 'y') => non_totalistic_closure!($t;_,$p,_,$p,$p,$p,_,$p),
			(5, 'q') => non_totalistic_closure!($t;_,_,$p,$p,$p,$p,$p,_),
			(5, 'j') => non_totalistic_closure!($t;_,_,$p,$p,_,$p,$p,$p),
			(5, 'r') => non_totalistic_closure!($t;_,_,$p,$p,$p,$p,_,$p),
			//6
			(6, 'c') => non_totalistic_closure!($t;_,$p,_,$p,$p,$p,$p,$p),
			(6, 'e') => non_totalistic_closure!($t;$p,_,$p,_,$p,$p,$p,$p),
			(6, 'k') => non_totalistic_closure!($t;_,$p,$p,$p,_,$p,$p,$p),
			(6, 'a') => non_totalistic_closure!($t;_,_,$p,$p,$p,$p,$p,$p),
			(6, 'i') => non_totalistic_closure!($t;$p,_,$p,$p,$p,$p,_,$p),
			(6, 'n') => non_totalistic_closure!($t;_,$p,$p,$p,$p,$p,$p,_),
			//7
			(7, 'c') => non_totalistic_closure!($t;_,$p,$p,$p,$p,$p,$p,$p),
			(7, 'e') => non_totalistic_closure!($t;$p,_,$p,$p,$p,$p,$p,$p),
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
			let is_not = s.starts_with("-");
			let s = if is_not {&s[1..]} else {s};
			let fs = s.chars().map(move|c|non_totalistic_closure!($t; $p, n, c));
			let fs: Vec<_> = fs.collect();
			(n, is_not, fs)
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
			for (n, is_not, fs) in &v {
				if *n == sum {
					if fs.is_empty() {
						return true;
					} else if *is_not {
						for f in fs {
							if f(c11,c12,c13,c21,c23,c31,c32,c33) {
								return false;
							}
						}
						return true;
					} else {
						for f in fs {
							if f(c11,c12,c13,c21,c23,c31,c32,c33) {
								return true;
							}
						}
						return false;
					}
				}
			}
			return false;
		}
	}};
}
