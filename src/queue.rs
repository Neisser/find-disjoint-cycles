use std::fmt::Debug;

/// A generic queue structure for holding items of any type that implements the `Debug` trait.
pub struct Queue<T: Debug> {
    pub items: Vec<T>,
}

impl<T: Debug> Queue<T> {
    /// Creates a new, empty queue.
    ///
    /// # Returns
    ///
    /// A new `Queue` instance with no items.
    pub fn new() -> Queue<T> {
        Queue {
            items: Vec::new(),
        }
    }

    /// Adds an item to the end of the queue.
    ///
    /// # Arguments
    ///
    /// * `item` - The item to add to the queue.
    pub fn enqueue(&mut self, item: T) {
        self.items.push(item);
    }

    /// Removes and returns the item from the front of the queue.
    ///
    /// # Returns
    ///
    /// An `Option` containing the item if the queue is not empty, or `None` if the queue is empty.
    pub fn dequeue(&mut self) -> Option<T> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items.remove(0))
        }
    }

    /// Checks if the queue is empty.
    ///
    /// # Returns
    ///
    /// `true` if the queue is empty, `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Displays the contents of the queue.
    pub fn display(&self) {
        println!("queue: {:?}", self.items);
    }
}
