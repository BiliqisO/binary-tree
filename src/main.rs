use std::collections::VecDeque;

fn main() {
    let mut node_a:Node<i32>= Node::new(3);
    let mut  node_b:Node<i32>= Node::new(11);
    let  mut node_c:Node<i32>= Node::new(4);
    let node_d:Node<i32>= Node::new(4);
    let node_e:Node<i32>= Node::new(2);
    let node_f:Node<i32>= Node::new(1);
    node_c.right(Some(Box::new(node_f)));
    node_b.left(Some(Box::new(node_d)));
    node_b.right(Some(Box::new(node_e)));
    node_a.left(Some(Box::new(node_b)));
    node_a.right(Some(Box::new(node_c)));

   let value = depth_first_value(&node_a);
   println!("value:{:?}", value);
   let value = breadth_first_value(&node_a);
   println!("value_queue:{:?}", value);
   let includes = tree_includes_breadth(&node_a, &String::from("x"));
   println!("includes:{:?}", includes);
   let tree_includes_recursive = tree_includes_recursive(&Some(Box::new(node_a.clone())), &String::from("e"));
   println!("tree_includes_recursive:{:?}", tree_includes_recursive);
   
   let tree_sum = tree_sum(&node_a);
   println!("tree_sum:{:?}", tree_sum);
   
}
//box because its recursive, hence possibly infinite,  storing the data(Node<T>) on heap because size is not fixed
#[derive(Debug, Clone)]
struct Node<T>{
    val: T,
    left:Option<Box<Node<T>>>,
    right:Option<Box<Node<T>>>,
    
}
impl<T> Node<T>{
    fn new(val:T) -> Node<T>{
     Node { val, left: None, right: None }
    }
    fn left(&mut self, left: Option<Box<Node<T>>>) {
        self.left = left;
        
    }
    fn right(&mut self, right: Option<Box<Node<T>>>)  {
        self.right = right;
        
    }
}
fn depth_first_value<T: std::fmt::Debug>(root: &Node<T>)-> Vec<&T>  {
    let mut stack:Vec<&Node<T>> = Vec::new();
    stack.push(root);
    let mut current_value:Vec<&T>  = Vec::new();
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
fn  breadth_first_value<T: std::fmt::Debug>(root: &Node<T>)-> Vec<&T>  {
    let mut stack:VecDeque<&Node<T>> = VecDeque::new();
     stack.push_back(root);
    let mut current_value:Vec<&T>  = Vec::new();
    while let Some(current) = stack.pop_front(){
      current_value.push(&current.val);
    if let Some(ref left) = &current.left {
    stack.push_back(left);
    }
    if let Some(ref right) = &current.right {
    stack.push_back(right);
    }
     }
     current_value
}
 
fn tree_includes_breadth<T: std::fmt::Debug>(root: &Node<T>, search_val:&String)-> bool {
 let mut stack:VecDeque<&Node<T>> = VecDeque::new();
    stack.push_back(root);
    while let Some(current) = stack.pop_front(){
    if format!("{:?}", current.val) == *search_val{return true;}
    if let Some(ref left) = &current.left {
    stack.push_back(left);
    }
    if let Some(ref right) = &current.right {
    stack.push_back(right);
    }
    };
    return false;
}
fn tree_includes_recursive<T: std::fmt::Debug>(root: &Option<Box<Node<T>>>, search_val:&String)-> bool {
    if let Some(node) = root {
        if format!("{:?}", node.val) == *search_val {
            return true;
        }
        return tree_includes_recursive(&node.left, search_val) || tree_includes_recursive(&node.right, search_val);
    }
    false
}
fn tree_sum<T>(root: &Node<T>) -> T 
where
    T: std::fmt::Debug + std::ops::Add<Output = T> + Copy + From<i32>,
{
    let mut stack: VecDeque<&Node<T>> = VecDeque::new();
    stack.push_back(root);
    let mut total_value: T = T::from(0);
    while let Some(current) = stack.pop_front() {
        total_value = total_value + current.val;
        if let Some(ref left) = &current.left {
            stack.push_back(left);
        }
        if let Some(ref right) = &current.right {
            stack.push_back(right);
        }
    }
    total_value
}