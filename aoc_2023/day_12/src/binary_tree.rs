use core::fmt;

pub struct BinaryTree {
    pub value: char,
    pub left: Option<Box<BinaryTree>>,
    pub right: Option<Box<BinaryTree>>,
}

impl BinaryTree {
    pub fn new(val: char) -> Box<BinaryTree> {
        Box::new(BinaryTree {
            value: val,
            left: None,
            right: None,
        })
    }
}

impl fmt::Debug for BinaryTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("BinaryTree")
            .field("value", &self.value)
            .field("left", &self.left.as_deref())
            .field("right", &self.right.as_deref())
            .finish()
    }
}
