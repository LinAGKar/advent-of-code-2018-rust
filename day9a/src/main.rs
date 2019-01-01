use std::cell::RefCell;
use std::io;
use std::rc::{Rc, Weak};

// Severely Incomplete and in some ways broken (e.g. head and tail are not updated by
// insert_after or pop_before) circular doubly linked list, just enough for this task
struct Node<T> {
    prev: Option<Weak<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>,
    value: T,
}

struct CircularList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

struct CircularListIterator<T> {
    item: Rc<RefCell<Node<T>>>,
}

impl<T> CircularList<T> {
    fn new() -> CircularList<T> {
        CircularList {
            head: None,
            tail: None,
        }
    }

    fn push_front(&mut self, value: T) {
        let item = Rc::new(RefCell::new(Node {
            prev: None,
            next: None,
            value: value,
        }));
        if let (Some(head), Some(tail)) = (&mut self.head, &self.tail) {
            item.borrow_mut().prev = Some(Rc::downgrade(tail));
            item.borrow_mut().next = Some(Rc::clone(head));
            head.borrow_mut().prev = Some(Rc::downgrade(&item));
            self.head = Some(Rc::clone(&item));
        } else {
            item.borrow_mut().prev = Some(Rc::downgrade(&item));
            item.borrow_mut().next = Some(Rc::clone(&item));
            self.head = Some(Rc::clone(&item));
            self.tail = Some(Rc::clone(&item));
        }
    }

    fn iter(&self) -> Option<CircularListIterator<T>> {
        if let Some(head) = &self.head {
            Some(CircularListIterator {
                item: Rc::clone(head),
            })
        } else {
            None
        }
    }
}

impl<T> CircularListIterator<T> {
    fn next(&mut self) -> Option<()> {
        let new_rc = if let Some(next) = &self.item.borrow().next {
            Rc::clone(next)
        } else {
            return None;
        };
        self.item = new_rc;
        Some(())
    }

    fn prev(&mut self) -> Option<()> {
        let new_rc = if let Some(prev) = &self.item.borrow().prev {
            prev.upgrade()?
        } else {
            return None;
        };
        self.item = new_rc;
        Some(())
    }

    fn insert_after(&mut self, value: T) -> Option<()> {
        let next_rc = if let Some(next) = &self.item.borrow().next {
            Rc::clone(next)
        } else {
            return None;
        };
        let this_rc = Rc::downgrade(&self.item);
        let item = Rc::new(RefCell::new(Node {
            prev: Some(this_rc),
            next: Some(next_rc),
            value: value,
        }));
        let mut was_single_node = false;
        let mut active_ref = self.item.borrow_mut();
        if let Some(next) = &active_ref.next {
            if let Ok(mut next_node) = next.try_borrow_mut() {
                next_node.prev = Some(Rc::downgrade(&item));
            } else {
                was_single_node = true;
            }
        } else {
            return None;
        };
        if was_single_node {
            active_ref.prev = Some(Rc::downgrade(&item));
        }
        active_ref.next = Some(Rc::clone(&item));
        Some(())
    }

    fn pop_before(&mut self) -> Option<T> {
        let prev_rc = if let Some(prev) = &self.item.borrow().prev {
            prev.upgrade().unwrap()
        } else {
            return None;
        };
        if Rc::ptr_eq(&prev_rc, &self.item) {
            return None;
        }
        let before_prev_rc = if let Some(before_prev) = &prev_rc.borrow().prev {
            before_prev.upgrade().unwrap()
        } else {
            return None;
        };
        before_prev_rc.borrow_mut().next = Some(Rc::clone(&self.item));
        self.item.borrow_mut().prev = Some(Rc::downgrade(&before_prev_rc));
        if let Ok(node) = Rc::try_unwrap(prev_rc) {
            Some(node.into_inner().value)
        } else {
            None
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut words = input.split_whitespace();
    let player_count: usize = words.next().unwrap().parse().unwrap();
    let end_value: u32 = words.nth(5).unwrap().parse().unwrap();

    let mut player_points = vec![0; player_count];
    let mut current_player = 0;

    let mut current_value = 0;
    let mut circle = CircularList::<u32>::new();
    circle.push_front(current_value);
    current_value += 1;
    let mut active = circle.iter().unwrap();
    while current_value <= end_value {
        if current_value % 23 == 0 {
            player_points[current_player] += current_value;
            for _ in 0..6 {
                active.prev().unwrap();
            }
            player_points[current_player] += active.pop_before().unwrap();
        } else {
            active.next().unwrap();
            active.insert_after(current_value).unwrap();
            active.next().unwrap();
        }
        current_value += 1;
        current_player = (current_player + 1) % player_count;
    }
    println!("{}", player_points.into_iter().max().unwrap());
}
