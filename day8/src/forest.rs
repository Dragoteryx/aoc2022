use std::fmt::{self, Debug};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Forest {
	trees: Vec<Vec<u8>>
}

impl Forest {
	pub fn new(trees: Vec<Vec<u8>>)-> Self {
		Self { trees }
	}

	pub fn tree(&self, x: usize, y: usize) -> Option<Tree> {
		if let Some(line) = self.trees.get(y) {
			if let Some(size) = line.get(x) {
				return Some(Tree {
					forest: self,
					size: *size,
					x, y
				});
			}
		}
		
		None
	}

	pub fn trees(&self) -> impl Iterator<Item = Tree> {
		self.trees.iter().enumerate().flat_map(move |(y, line)| {
			line.iter().enumerate().map(move |(x, size)| {
				Tree {
					forest: self,
					size: *size,
					x, y
				}
			})
		})
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tree<'f> {
	forest: &'f Forest,
	size: u8,
	x: usize,
	y: usize
}

impl<'f> Tree<'f> {
	pub fn forest(&self) -> &'f Forest {
		self.forest
	}

	pub fn size(&self) -> u8 {
		self.size
	}

	pub fn pos(&self) -> (usize, usize) {
		(self.x, self.y)
	}

	pub fn top_trees(&self) -> impl Iterator<Item = Tree> {
		(0..self.y).rev().map_while(|y| self.forest.tree(self.x, y))
	}

	pub fn bottom_trees(&self) -> impl Iterator<Item = Tree> {
		(self.y..).skip(1).map_while(|y| self.forest.tree(self.x, y))
	}

	pub fn left_trees(&self) -> impl Iterator<Item = Tree> {
		(0..self.x).rev().map_while(|x| self.forest.tree(x, self.y))
	}

	pub fn right_trees(&self) -> impl Iterator<Item = Tree> {
		(self.x..).skip(1).map_while(|x| self.forest.tree(x, self.y))
	}

	pub fn is_visible_from_top(&self) -> bool {
		self.top_trees().all(|tree| tree.size < self.size)
	}

	pub fn is_visible_from_bottom(&self) -> bool {
		self.bottom_trees().all(|tree| tree.size < self.size)
	}

	pub fn is_visible_from_left(&self) -> bool {
		self.left_trees().all(|tree| tree.size < self.size)
	}

	pub fn is_visible_from_right(&self) -> bool {
		self.right_trees().all(|tree| tree.size < self.size)
	}

	pub fn is_visible(&self) -> bool {
		self.is_visible_from_top()
		|| self.is_visible_from_bottom()
		|| self.is_visible_from_left()
		|| self.is_visible_from_right()
	}

	pub fn scenic_score(&self) -> usize {
		fn take_while_include<T>(iter: impl Iterator<Item = T>, mut func: impl FnMut(&T) -> bool) -> impl Iterator<Item = T> {
			let mut items = Vec::new();
			for item in iter {
				let ok = func(&item);
				items.push(item);
				if !ok { break; }
			}

			items.into_iter()
		}

		let top = take_while_include(self.top_trees(), |tree| tree.size < self.size).count();
		let bottom = take_while_include(self.bottom_trees(), |tree| tree.size < self.size).count();
		let left = take_while_include(self.left_trees(), |tree| tree.size < self.size).count();
		let right = take_while_include(self.right_trees(), |tree| tree.size < self.size).count();
		top * bottom * left * right
	}
}

impl Debug for Tree<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("Tree")
			.field("x", &self.x)
			.field("y", &self.y)
			.field("size", &self.size)
			.finish()
	}
}