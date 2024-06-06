use std::fmt::Debug;

#[derive(Debug)]
pub struct Queue<T: Debug> {
  pub items: Vec<T>
}

impl<T: Debug> Queue<T>{
  pub fn new() -> Queue<T> {
    Queue {
      items: Vec::new()
    }
  }

  pub fn enqueue(&mut self, item: T) {
    self.items.push(item);
  }

  pub fn dequeue(&mut self) -> Option<T> {
    if self.items.is_empty() {
      None
    } else {
      Some(self.items.remove(0))
    }
  }

  pub fn is_empty(&self) -> bool {
    self.items.is_empty()
  }
  
  pub fn display(&self) {
    println!("queue: {:?}", self.items);
  }

  // pub fill_queue(&mut self, op: T, params: P){
  //   op(&mut self, param)
  // }
}