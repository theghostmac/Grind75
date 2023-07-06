/ Define a struct for the linked list node.
struct ListNode<T> {
    payload: T,
    next: Option<Box<ListNode<T>>>,
}

// Define methods on the ListNode struct.
impl<T> ListNode<T> {
    // constructor for creating a new node.
    fn new(payload: T) -> Self {
        ListNode {
            payload: (),
            next: None,
        }
    }
}

// Define the LinkedList struct.
struct LinkedList<T> {
    head: Option<Box<ListNode<T>>>,
    tail: Option<Box<ListNode<T>>>,
}

// Define methods on the LinkedList struct.
impl<T> LinkedList<T> {
    // constructor for creating an empty linked list.
    fn new(payload: T) -> Self {
        LinkedList {
            head: None,
            tail: None,
        }
    }

    // method to check if linked list is empty.
    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    // method to insert a value at the end of the linked list.
    fn push(&mut self, payload: T) {
        let new_node = Box::new(ListNode::new(payload));
        let raw_node = Box::into_raw(new_node);

        unsafe {
            if let Some(tail) = self.tail.as_mut() {
                *tail.next = Some(Box::from_raw(raw_node));
            } else {
                self.head = Some(Box::from_raw(raw_node));
            }
            self.tail = Some(raw_node);
        }
    }

    // method to pop out and return the first value in the list.
    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|boxed_node| {
            let node = *boxed_node;
            self.head = node.next;
            if self.head.is_none() {
                self.tail = None;
            }
            node.payload
        })
    }
}