use std::fmt::Debug;
use std::fmt::Formatter;
use super::RuleType;

#[derive(PartialEq, Eq, Clone, Debug)]
enum RuleTreeNode<T> {
	Leaf(usize, Vec<T>),
	NotLeaf(usize, Vec<usize>),
}

pub struct RuleTree<T> {
	rules: Vec<RuleTreeNode<T>>,
	states: Vec<T>,
	params: [T; 9]
}

impl<T: Clone> RuleTree<T> {
	pub fn new(ss: Vec<T>) -> Self {
		let d = ss.first().unwrap();
		RuleTree {
			rules: Vec::new(),
			params: [
				d.clone(), d.clone(), d.clone(),
				d.clone(), d.clone(), d.clone(),
				d.clone(), d.clone(), d.clone(),
			],
			states: ss,
		}
	}
}
impl<T: Eq> Debug for RuleTree<T> {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		writeln!(f, "num_states={}", self.states.len())?;
		writeln!(f, "num_neighbors=8")?;
		writeln!(f, "num_nodes={}", self.rules.len())?;
		for rule in &self.rules {
			match rule {
				RuleTreeNode::Leaf(height, children) => {
					write!(f, "{}", height)?;
					for c in children {
						let pos = self.states
							.iter()
							.position(|x| x == c)
							.unwrap();
						write!(f, " {}", pos)?;
					}
					writeln!(f, "")?;
				},
				RuleTreeNode::NotLeaf(height, children) => {
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
	fn get_node(&mut self, n: &RuleTreeNode<T>) -> usize {
		let mut iter = self.rules.iter();
		match iter.position(|x| x == n) {
			Some(n) => n,
			None => {
				self.rules.push(n.clone());
				self.rules.len() - 1
			}
		}
	}
	fn recur<F: RuleType<T>>(&mut self, height: usize, f: &F) -> usize {
		if height == 1 {
			let mut children: Vec<T> = Vec::new();
			for c in &self.states {
				self.params[8] = c.clone();
				/*params的参数顺序为：nw、ne、sw、se、n、w、e、s、c*/
				children.push(f(
					&self.params[0],&self.params[4],&self.params[1],
					&self.params[5],&self.params[8],&self.params[6],
					&self.params[2],&self.params[7],&self.params[3]));
			}
			self.get_node(&RuleTreeNode::Leaf(1, children))
		} else {
			let mut children: Vec<usize> = Vec::new();
			for c in &self.states.clone() {
				self.params[9-height] = c.clone();
				children.push(self.recur(height-1, f));
			}
			self.get_node(&RuleTreeNode::NotLeaf(height, children))
		}
	}
	pub fn create_tree<F: RuleType<T>>(&mut self, f: &F) {
		self.rules.clear();
		self.recur(9, f);
	}
	pub fn create_von_neumann_tree<F: RuleType<T>>(&mut self, f: &F) {
		self.rules.clear();
		self.recur(5, f);
	}
}
