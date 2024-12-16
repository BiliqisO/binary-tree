fn main() {
    let mut node_a:Node<String>= Node::new(String::from("a"));
    let mut  node_b:Node<String>= Node::new(String::from("b"));
    let  mut node_c:Node<String>= Node::new(String::from("c"));
    let node_d:Node<String>= Node::new(String::from("d"));
    let node_e:Node<String>= Node::new(String::from("e"));
    let node_f:Node<String>= Node::new(String::from("f"));
    node_c.right(Some(Box::new(node_f)));
    node_b.left(Some(Box::new(node_d)));
    node_b.right(Some(Box::new(node_e)));
    node_a.left(Some(Box::new(node_b)));
    node_a.right(Some(Box::new(node_c)));

   let value = depth_first_value(&node_a);
   println!("value:{:?}", value)
}
//box because its recursive, hence possibly infinite,  storing the data(Node<T>) on heap because size is not fixed
//phantomdata
#[derive(Debug, Clone)]
struct Node<T>{
    val: String,
    left:Option<Box<Node<T>>>,
    right:Option<Box<Node<T>>>,
    phantom: std::marker::PhantomData<T>,
}
impl<T> Node<T>{
    fn new(val:String) -> Node<T>{
     Node { val, left: None, right: None, phantom: std::marker::PhantomData::<T> }
    }
    fn left(&mut self, left: Option<Box<Node<T>>>) {
        self.left = left;
        
    }
    fn right(&mut self, right: Option<Box<Node<T>>>)  {
        self.right = right;
        
    }
}
fn depth_first_value<T: std::fmt::Debug>(root: &Node<T>)-> Vec<&String>  {
    let mut stack:Vec<&Node<T>> = Vec::new();
    stack.push(root);
    let mut current_value:Vec<&String>  = Vec::new();
    println!("stack12, {:?}", stack);
    while let Some(current) = stack.pop(){
    current_value.push(&current.val);
    if let Some(ref left) = &current.left {
    stack.push(left);
    }
    if let Some(ref right) = &current.right {
    stack.push(right);
    }
    }
    current_value

}