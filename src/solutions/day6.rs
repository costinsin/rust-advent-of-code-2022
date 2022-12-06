use std::{
    collections::{HashSet, LinkedList},
    error::Error,
};

/// A queue that holds up to self.size elements.
/// If an element is pushed when the queue is full, the earliest inserted
/// element is popped.
struct SpecialQueue<T>
where
    T: Eq + std::hash::Hash,
{
    queue: LinkedList<T>,
    size: usize,
}

impl<T: Eq + std::hash::Hash> SpecialQueue<T> {
    pub fn new(size: usize) -> Self {
        return SpecialQueue {
            queue: LinkedList::new(),
            size,
        };
    }

    /// Checks if there are any duplicates in the queue
    pub fn contains_duplicates(&self) -> bool {
        let mut set = HashSet::new();

        for e in &self.queue {
            if set.contains(e) {
                return true;
            }
            set.insert(e);
        }

        false
    }

    /// Checks if the queue has reached it's maximum size
    pub fn is_full(&self) -> bool {
        self.queue.len() == self.size
    }

    /// Pushes back a new element, if the max size is reached,
    /// pops front the first element and returns it
    pub fn push_back(&mut self, elem: T) -> Option<T> {
        self.queue.push_back(elem);

        if self.queue.len() > self.size {
            self.queue.pop_front()
        } else {
            None
        }
    }
}

#[allow(dead_code)]
pub fn part1(input: &String) -> Result<(), Box<dyn Error>> {
    let mut index = 0;
    let mut special_queue = SpecialQueue::new(4);

    for c in input.chars() {
        index += 1;
        special_queue.push_back(c);
        if special_queue.is_full() && !special_queue.contains_duplicates() {
            break;
        }
    }

    println!("{}", index);

    Ok(())
}

#[allow(dead_code)]
pub fn part2(input: &String) -> Result<(), Box<dyn Error>> {
    let mut index = 0;
    let mut special_queue = SpecialQueue::new(14);

    for c in input.chars() {
        index += 1;
        special_queue.push_back(c);
        if special_queue.is_full() && !special_queue.contains_duplicates() {
            break;
        }
    }

    println!("{}", index);

    Ok(())
}
