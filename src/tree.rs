use std::fmt;
use std::iter::Peekable;
use std::str::Chars;
use std::string::ToString;

pub struct Tree {
    root: Node,
}

impl Tree {
    pub fn from_string(s: &str) -> Self {
        Tree {
            root: Node::from_chars(&mut s.chars().peekable()),
        }
    }
}

impl PartialEq for Tree {
    fn eq(&self, other: &Tree) -> bool {
        self.root == other.root
    }
}

impl ToString for Tree {
    fn to_string(&self) -> String {
        self.root.to_string()
    }
}

impl fmt::Debug for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

struct Node {
    val: String,
    children: Vec<Node>,
}

impl Node {
    fn new(val: &str, children: Vec<Node>) -> Self {
        Node {
            val: val.to_owned(),
            children: children,
        }
    }

    fn from_chars(mut chars: &mut Peekable<Chars>) -> Self {
        let mut val = String::new();
        let mut children = vec![];
        while let Some(&c) = chars.peek() {
            if c.is_whitespace() {
                Node::next_token(&mut chars);
            } else if c == '(' {
                Node::next_token(&mut chars);
                if !val.is_empty() {
                    children.push(Node::from_chars(&mut chars));
                }
            } else if c == ')' {
                Node::next_token(&mut chars);
                break;
            } else {
                let label = Node::next_token(&mut chars);
                if val.is_empty() {
                    val = label;
                } else {
                    children.push(Node::new(&label, vec![]));
                }
            }
        }
        Node::new(&val, children)
    }

    fn next_token(chars: &mut Peekable<Chars>) -> String {
        let c = *chars.peek().unwrap();
        let start_on_paren = c == '(' || c == ')';
        let mut hit_whitespace = c.is_whitespace();
        let mut passed = String::new();
        passed.push(c);
        chars.next();
        while let Some(&c) = chars.peek() {
            if c.is_whitespace() {
                hit_whitespace = true;
                chars.next();
                continue;
            }
            if c == '(' || c == ')' || hit_whitespace || start_on_paren {
                break;
            }
            passed.push(c);
            chars.next();
        }
        passed.trim().to_owned()
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.val == other.val && self.children == other.children
    }
}

impl ToString for Node {
    fn to_string(&self) -> String {
        if self.children.is_empty() {
            self.val.to_owned()
        } else {
            let child_string = self.children
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(" ");
            format!("({} {})", self.val, child_string)
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! node {
        ($val:ident) => (Node::new(stringify!($val), vec![]));
        (($val:ident $($child:tt)+)) => (Node::new(stringify!($val), vec![$(node!($child)),+]));
        ([$val:expr]) => (Node::new($val, vec![]));
        (([$val:expr] $($child:tt)+)) => (Node::new($val, vec![$(node!($child)),+]));
    }

    macro_rules! tree {
        ($node:tt) => (Tree { root: node!($node) } );
    }

    #[test]
    fn tree_from_string_works() {
        assert_eq!(Tree::from_string("a"), tree!(a));
        assert_eq!(Tree::from_string("(a b)"), tree!((a b)));
        assert_eq!(Tree::from_string("(a b c)"), tree!((a b c)));
        assert_eq!(Tree::from_string("(a b (c d))"), tree!((a b (c d))));
        assert_eq!(Tree::from_string("(a b (c d e))"), tree!((a b (c d e))));
        assert_eq!(
            Tree::from_string("(a (b c d) (e f g))"),
            tree!((a (b c d) (e f g)))
        );
        assert_eq!(
            Tree::from_string("(a (b (c d (e f)) (g h)) (i (j k) (l m n)))"),
            tree!((a (b (c d (e f)) (g h)) (i (j k) (l m n))))
        );

        assert_eq!(Tree::from_string("-a-"), tree!(["-a-"]));
        assert_eq!(
            Tree::from_string(
                "(-a- (-b- (-c- -d- (-e- -f-)) (-g- -h-)) (-i- (-j- -k-) (-l- -m- -n-)))"
            ),
            tree!(
                (["-a-"](["-b-"](["-c-"]["-d-"](["-e-"]["-f-"]))(["-g-"]["-h-"]))(
                    ["-i-"](["-j-"]["-k-"])(["-l-"]["-m-"]["-n-"])
                ))
            )
        );
    }
}
