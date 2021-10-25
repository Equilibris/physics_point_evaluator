use crate::tree::Node;

pub trait Invertible<T: Node>: Node {
    fn get_inverse(&self) -> T;
}

pub struct Add<A: Node, B: Node> {
    a: A,
    b: B,
}
pub struct Sub<A: Node, B: Node> {
    a: A,
    b: B,
}

pub struct Mul<A: Node, B: Node> {
    a: A,
    b: B,
}
pub struct Div<A: Node, B: Node> {
    a: A,
    b: B,
}

impl<A: Node, B: Node> Node for Add<A, B> {
    fn evaluate(&self) -> Vec<f64> {
        let a = self.a.evaluate();
        let b = self.b.evaluate();

        if a.len() == 0 || b.len() == 0 {
            Vec::new()
        } else {
            a.into_iter()
                .flat_map(|v| b.clone().into_iter().map(move |x| x + v))
                .collect()
        }
    }
}
impl<A: Node, B: Node> Node for Sub<A, B> {
    fn evaluate(&self) -> Vec<f64> {
        let a = self.a.evaluate();
        let b = self.b.evaluate();

        if a.len() == 0 || b.len() == 0 {
            Vec::new()
        } else {
            a.into_iter()
                .flat_map(|v| b.clone().into_iter().map(move |x| x - v))
                .collect()
        }
    }
}
