use std::fmt::Debug;
use std::fmt::Formatter;
use std::collections::BTreeMap;
use super::RuleType;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
struct RuleTreeNode {
	height: usize,
	children: Vec<usize> //height为1时为state的序号，大于1时为节点序号
}

pub struct RuleTree<T> {
	rules: Vec<RuleTreeNode>,
	map: BTreeMap<RuleTreeNode, usize>,//内容与rules一致，用于加速
	states: Vec<T>,
	params: [T; 9],
	neighbors: usize,
}

impl<T: Clone> RuleTree<T> {
	pub fn new(ss: Vec<T>) -> Self {
		let d = ss.first().unwrap();
		RuleTree {
			rules: Vec::with_capacity(10000),
			map: BTreeMap::new(),
			params: [
				d.clone(), d.clone(), d.clone(),
				d.clone(), d.clone(), d.clone(),
				d.clone(), d.clone(), d.clone(),
			],
			states: ss,
			neighbors: 0,
		}
	}
}
impl<T: Eq> Debug for RuleTree<T> {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		writeln!(f, "num_states={}", self.states.len())?;
		writeln!(f, "num_neighbors={}", self.neighbors)?;
		writeln!(f, "num_nodes={}", self.rules.len())?;
		for rule in &self.rules {
			match rule {
				RuleTreeNode{height, children} => {
					write!(f, "{}", height)?;
					for c in children {
						write!(f, " {}", c)?;
					}
					writeln!(f, "")?;
				}
			};
		}
		write!(f, "")
	}
}
impl<T: Eq + Clone> RuleTree<T> {
	fn get_node(&mut self, node: &RuleTreeNode) -> usize {
		match self.map.get(node) {
			Some(pos) => *pos,
			None => {
				self.rules.push(node.clone());
				let pos = self.rules.len() - 1;
				self.map.insert(node.clone(), pos);
				pos
			}
		}
	}
	fn recur<F: RuleType<T>>(&mut self, height: usize, f: &F) -> usize {
		let len = self.states.len();
		let mut children: Vec<usize> = Vec::with_capacity(len);
		if height == 1 {
			for c in &self.states {
				self.params[8] = c.clone();
				/*params的参数顺序为：nw、ne、sw、se、n、w、e、s、c*/
				let s = f(
					&self.params[0],&self.params[4],&self.params[1],
					&self.params[5],&self.params[8],&self.params[6],
					&self.params[2],&self.params[7],&self.params[3]);
				children.push(self.states.iter().position(|x| x == &s).unwrap());
			}
			self.get_node(&RuleTreeNode{height: 1, children: children})
		} else {
			for c in &self.states.clone() {
				self.params[9-height] = c.clone();
				children.push(self.recur(height-1, f));
			}
			self.get_node(&RuleTreeNode{height: height, children: children})
		}
	}
	pub fn create_tree<F: RuleType<T>>(&mut self, f: &F) {
		self.rules.clear();
		self.map.clear();
		self.neighbors = 8;
		self.recur(9, f);
	}
	pub fn create_von_neumann_tree<F: RuleType<T>>(&mut self, f: &F) {
		self.rules.clear();
		self.map.clear();
		self.neighbors = 4;
		self.recur(5, f);
	}
}
