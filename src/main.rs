// Make a big array
// Get a magic function that converts elements into integers (hashing)
// Store elements in the array at the index specified by their magic integer
// Do something interesting if two elements get the same index (hash conflict) (robinhood hashmap)

static ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

type Link<T> = Option<Box<Node<T>>>;

pub struct HashMap<T> {
    head: Link<T>,
    size: i32,
}

impl<T> HashMap<T> {
    pub fn new() -> Self {
        HashMap {
            head: None,
            size: 0,
        }
    }

    //only for String
    pub fn push(&mut self, data: T) {
        let hash_key = self.get_hash_key(data);
        let new_node = Box::new(Node {
            data: data,
            next: self.head.take(),
        });

        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }
    fn get_hash_key(&self, data: T) -> i32 {
        let mut key = 0;
        for c in data.chars() {
            for i in 0..ALPHABET.len() {
                if c == ALPHABET[i] {
                    key += i as i32;
                }
            }
        }
        key % self.size
    }
}

impl HashMap<T> {}

impl<T> Drop for HashMap<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

pub struct Node<T> {
    key: i32,
    data: T,
    next: Link<T>,
}

fn main() {}

#[cfg(test)]
mod test {
    use super::HashMap;
    #[test]
    fn basics() {
        let mut list = HashMap::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
