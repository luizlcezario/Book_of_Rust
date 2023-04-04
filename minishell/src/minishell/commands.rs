#[derive(PartialEq)]
pub enum ParseTypes {
	Word,
	Pipe,
	Redirection,
	End,
}

pub struct ElementLine {
	parse_type: ParseTypes,
	value: String,
}

impl ElementLine {
	fn new() -> ElementLine {
		ElementLine  {
			parse_type: ParseTypes::End,
			value: String::new(),
		}
	}
}

pub struct ParsedNode {
	element: ElementLine,
	right: Option<Box<ParsedNode>>,
	left:  Option<Box<ParsedNode>>,
}

impl ParsedNode {
	fn new(element: ElementLine) -> ParsedNode {
		ParsedNode {
			element, 
			right: None,
			left: None,
		}
	}
	pub fn setLeft(mut self, node: ParsedNode) -> Self {
        self.left = Some(Box::new(node));
        self
    }

    pub fn setRight(mut self, node: ParsedNode) -> Self {
        self.right = Some(Box::new(node));
        self
    }

}

struct ParsedTree {
	root: Option<Box<ParsedNode>>,
	last_added: &Option<&Box<&ParsedNode>>,
}

impl ParsedTree {
	fn new() -> ParsedTree {
		ParsedTree {
			root: None,
			last_added: &None,
		}
	}
	fn add_element(&mut self, element: ElementLine) {
		match self.root {
			None => {
				self.root = Some(ParsedNode::new(element));
				return;
			},
			Some(mut r) => self.insert(& mut r, ParsedNode::new(element)),
		}
	}
	fn insert(&mut self, &mut now: &mut Option<Box<ParsedNode>>, newElement: ParsedNode) {
		if(now.is_none()) {
			now = Some(Box::new(newElement));
			return;
		}

		21                                                                                               
		match newElement.element.parse_type {
			ParseTypes::Redirection => {
				self.insert(&mut now.left, newElement);
				self.last_added = &now.left;
			},
			ParseTypes::Pipe => { 
				if (now.)
				self.insert(&mut now.right, newElement);
				self.last_added = &now.right;
			},
			ParsedTree::Word => {
				if (self.last_added.is_none()) {
					self.insert(&mut now.right, newElement);
					self.last_added = &now.right;
				} else {
					self.insert(&mut now.left, newElement);
					self.last_added = &now.left;
				}
				self.last_added = &now.left;
			},
			};
		}
	}
	// fn showInOrder( now: Option<&ParsedNode>, i: i32) {
		if (!now.is_none()) {
			ParsedTree::showInOrder(now.getLeft());
			i += 1;
			println!("[{}]: {}", i, now.);
			ParsedTree::showInOrder(now.getRight());
		}
	}
}