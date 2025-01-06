pub struct Tree {
    value: i32,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            value: 0,
            left: None,
            right: None,
        }
    }

    pub fn set_value(&mut self, value: i32) {
        self.value = value;
    }

    pub fn search(node: &Tree, value: i32) -> bool {
        if node.value == value {
            return true;
        }

        if value < node.value {
            if let Some(left) = &node.left {
                return Tree::search(left, value);
            }
        } else {
            if let Some(right) = &node.right {
                return Tree::search(right, value);
            }
        }

        false
    }

    pub fn add(&mut self, value: i32) {
        if value < self.value {
            if let Some(left) = &mut self.left {
                left.add(value);
            } else {
                self.left = Some(Box::new(Tree {
                    value,
                    left: None,
                    right: None,
                }));
            }
        } else if value > self.value {
            if let Some(right) = &mut self.right {
                right.add(value);
            } else {
                self.right = Some(Box::new(Tree {
                    value,
                    left: None,
                    right: None,
                }));
            }
        }
    }

    pub fn presentation(&self) -> String {
        let mut result = String::new();

        // Process left subtree
        if let Some(left) = &self.left {
            result.push_str(&format!("({}) <- ", left.presentation()));
        }

        // Process current node
        result.push_str(&format!("{}", self.value));

        // Process right subtree
        if let Some(right) = &self.right {
            result.push_str(&format!(" -> ({})", right.presentation()));
        }

        result
    }
}
