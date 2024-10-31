#[derive(Debug)]
pub struct Node {
    parent: * const Node,
    children: Vec<Box<Node>>,
    value: i32,
}

impl Node {

    pub fn new(value: i32) -> Self {
        Self {
            parent: std::ptr::null(),
            children: Vec::new(),
            value,
        }
    }

    pub fn add_child(&mut self, mut child: Node) {
        child.parent = self as * const Node;
        self.children.push(Box::new(child));
    }

    pub fn remove_child(&mut self, index: usize) {
        let child = self.children.get_mut(index).unwrap();
        child.parent = std::ptr::null();
        self.children.remove(index);
    }

    pub fn child(&self, index: usize) -> &Node {
        self.children.get(index).unwrap()
    }

    pub fn child_mut(&mut self, index: usize) -> &mut Node {
        self.children.get_mut(index).unwrap()
    }

    pub fn parent(&self) -> Option<&Node> {
        if self.parent.is_null() {
            None
        } else {
            unsafe { Some(&*self.parent) }
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }

    pub fn set_value(&mut self, value: i32) {
        self.value = value;
    }
}

// main
// let mut root = Box::new(Node::new(5));
// root.add_child(Node::new(10));
// root.add_child(Node::new(15));
// let first = root.child(0);
// println!("see first parent: {:?}", first.parent());