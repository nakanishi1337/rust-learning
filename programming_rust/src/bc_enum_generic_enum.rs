// ノード（列挙型）
// 重要: Box<Node<T>>を使うことで再帰的な型が実装可能になる
// Boxはヒープ上にデータを配置し、サイズが固定される（ポインタのサイズ）
// もしBox無しで Node(T, Node<T>, Node<T>) と書くと、
// コンパイラは無限にサイズを計算しようとしてエラーになる
#[derive(Debug, PartialEq)]
pub enum Node<T> {
    Empty,
    Leaf(T),
    Branch(T, Box<Node<T>>, Box<Node<T>>),
}

// ツリー（構造体）
#[derive(Debug)]
pub struct Tree<T> {
    pub root: Node<T>,
}

impl<T> Tree<T> {
    // 空のツリーを作成
    pub fn new() -> Self {
        Tree { root: Node::Empty }
    }

    // 葉ノードを持つツリーを作成
    pub fn leaf(value: T) -> Self {
        Tree {
            root: Node::Leaf(value),
        }
    }

    // ブランチを持つツリーを作成
    pub fn branch(value: T, left: Node<T>, right: Node<T>) -> Self {
        Tree {
            root: Node::Branch(value, Box::new(left), Box::new(right)),
        }
    }
}

impl<T: std::fmt::Display> Tree<T> {
    // ツリーを表示
    pub fn print(&self) {
        self.print_node(&self.root, 0);
    }

    fn print_node(&self, node: &Node<T>, depth: usize) {
        let indent = "  ".repeat(depth);
        match node {
            Node::Empty => println!("{}(空)", indent),
            Node::Leaf(v) => println!("{}葉: {}", indent, v),
            Node::Branch(v, left, right) => {
                println!("{}枝: {}", indent, v);
                self.print_node(left, depth + 1);
                self.print_node(right, depth + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        let tree: Tree<i32> = Tree::new();
        println!("空のツリー:");
        tree.print();
        assert_eq!(tree.root, Node::Empty);
    }

    #[test]
    fn test_leaf() {
        let tree = Tree::leaf(42);
        println!("\n葉のみのツリー:");
        tree.print();
        assert_eq!(tree.root, Node::Leaf(42));
    }

    #[test]
    fn test_simple_tree() {
        // シンプルなツリー
        //       5
        //      / \
        //     3   8
        let tree = Tree::branch(5, Node::Leaf(3), Node::Leaf(8));

        println!("\nシンプルなツリー:");
        tree.print();
    }

    #[test]
    fn test_nested_tree() {
        // ネストしたツリー
        //         10
        //        /  \
        //       5    15
        //      / \
        //     3   7
        let tree = Tree::branch(
            10,
            Node::Branch(5, Box::new(Node::Leaf(3)), Box::new(Node::Leaf(7))),
            Node::Leaf(15),
        );

        println!("\nネストしたツリー:");
        tree.print();
    }

    #[test]
    fn test_string_tree() {
        let tree = Tree::branch("親", Node::Leaf("左の子"), Node::Leaf("右の子"));

        println!("\n文字列ツリー:");
        tree.print();
    }
}
