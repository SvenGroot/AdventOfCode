use std::fmt::Display;

// There are existing tree crates; I wanted to try building one.
pub struct TreeNode<T> {
    content: T,
    children: Vec<TreeNode<T>>,
}

impl<T> TreeNode<T> {
    pub fn new(content: T) -> Self {
        Self {
            content,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, content: T) -> &mut Self {
        self.children.push(Self::new(content));
        self.children.last_mut().unwrap()
    }

    pub fn children(&self) -> impl Iterator<Item = &Self> {
        self.children.iter()
    }

    pub fn children_mut(&mut self) -> impl Iterator<Item = &mut Self> {
        self.children.iter_mut()
    }

    pub fn walk(&self, f: &mut impl FnMut(&T)) {
        f(&self.content);
        for child in self.children.iter() {
            child.walk(f);
        }
    }

    pub fn walk_mut(&mut self, f: &mut impl FnMut(&mut T)) {
        f(&mut self.content);
        for child in self.children.iter_mut() {
            child.walk_mut(f);
        }
    }

    pub fn content(&self) -> &T {
        &self.content
    }

    pub fn content_mut(&mut self) -> &mut T {
        &mut self.content
    }

    pub fn find_mut(&mut self, mut pred: impl FnMut(&T) -> bool) -> Option<&mut Self> {
        self.children.iter_mut().find(|node| pred(node.content()))
    }
}

impl<T: Display> TreeNode<T> {
    fn print_helper(&self, indent: usize, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:>indent$} {}", "-", &self.content, indent = indent)?;
        for child in self.children() {
            child.print_helper(indent + 2, f)?;
        }

        Ok(())
    }
}

impl<T: Display> Display for TreeNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print_helper(2, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node() {
        let mut root = TreeNode::new(1);
        let node = root.add_child(2);
        node.add_child(3);
        root.add_child(4);
        let mut sum = 0;
        root.walk(&mut |val| sum += val);
        assert_eq!(10, sum);
        root.walk_mut(&mut |val| *val += 1);
        assert_eq!(3, *(root.children().next().unwrap().content()));
    }
}
