pub struct QuadTree<T> {
    values: Vec<T>,
    root: QuadTreeNode,
    placer: Box<dyn TreeNodePlacer<T>>,
}

pub struct QuadTreeNode {
    pub depth: usize,
    pub semantic: QuadTreeNodeSemantic
}

pub enum QuadTreeNodeSemantic {
    Tree(Box<[QuadTreeNode; 4]>),
    Value(usize),
    Void,
}

pub enum TreeNodePosition {
    TopRight = 0,
    BottomRight = 1,
    BottomLeft = 2,
    TopLeft = 3,
}

trait TreeNodePlacer<T: ?Sized> {
    fn place(&self, v: &T, rank: usize) -> TreeNodePosition;
}

impl<T> QuadTree<T> {
    pub fn new(placer: Box<dyn TreeNodePlacer<T>>) -> Self {
        Self {
            values: Vec::new(),
            root: QuadTreeNode {
                depth: 0,
                semantic: QuadTreeNodeSemantic::Void,
            },
            placer,
        }
    }

    pub fn update_value(&mut self, v: T, index: usize) {
        self.values[index] = v;

        todo!();
        // self.placer.place(v, rank)

        // todo: normalize tree;
    }
}