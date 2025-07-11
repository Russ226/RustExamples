use std::mem;


#[derive(PartialEq, Default)]
struct SinglyLinkListNode<T>{
    val: T,
    next: Option<Box<SinglyLinkListNode<T>>>
}

impl<T> SinglyLinkListNode<T>{
    fn insert_next(&mut self, node: SinglyLinkListNode<T>){
        match &self.next {
            None => self.next = Some(Box::new(node)),
            Some(n ) => {
                drop(n);
                self.next = Some(Box::new(node))
            }
        }
    }

    fn getNext(&mut self) -> Option<Box<SinglyLinkListNode<T>>> {
        
        return match self.next {
            None => None,
            Some(mut n) => Some(mem::take(&mut n))
        }
    }
}

struct SinglyLinkList<T>{
    length: u16,
    head: Box<SinglyLinkListNode<T>>
}

impl<T> SinglyLinkList<T>{
    fn create_singly_link_list(val: T) -> Box<SinglyLinkList<T>>{
        let new_head = Box::new(SinglyLinkListNode {val: val, next: None });
        return Box::new(SinglyLinkList { length: 1, head: new_head });
    }

    fn insert_val(&mut self, val: T){
        let mut cur = &mut self.head;

        loop{
            let _next = match cur.getNext() {
                None => break
           };

        }

        cur.next = Some(Box::new(SinglyLinkListNode {val: val, next: None }));
    }
}

fn main() {
    let mut list = SinglyLinkList::create_singly_link_list(5);
    list.insert_val(3);

    println!("the values of head: {}", list.head.val);
    println!("the values of next: {}", list.head.next.unwrap().val);
}
