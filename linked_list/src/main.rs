use std::rc::Rc;

#[derive(PartialEq, Default)]
struct SinglyLinkListNode<T>{
    val: T,
    next: Option<Rc<SinglyLinkListNode<T>>>
}

// impl<T> SinglyLinkListNode<T>{
//     fn insert_next(&mut self, node: SinglyLinkListNode<T>){
//         match &self.next {
//             None => self.next = Some(Box::new(node)),
//             Some(n ) => {
//                 drop(n);
//                 self.next = Some(Box::new(node))
//             }
//         }
//     }

//     fn getNext(&mut self) -> Option<Box<SinglyLinkListNode<T>>> {
        
//         return match self.next {
//             None => None,
//             Some(mut n) => Some(mem::take(&mut n))
//         }
//     }
// }

struct SinglyLinkList<T>{
    length: u16,
    head: Rc<SinglyLinkListNode<T>>
}

impl<T> SinglyLinkList<T>{
    fn create_singly_link_list(val: T) -> SinglyLinkList<T>{
        let new_head = Rc::new(SinglyLinkListNode {val: val, next: None });
        return SinglyLinkList { length: 1, head: new_head };
    }

    fn insert_val(&mut self, val: T){
        let mut cur = Rc::downgrade(&mut self.head);

        loop{
            let mut cur_str_pointer = cur.upgrade().unwrap().clone();
            let node = Rc::get_mut(&mut cur_str_pointer).unwrap();
            match &node.next{
                None => break,
                Some(n) => cur = Rc::downgrade(&n)
            };
        }

        let mut binding = cur.upgrade().unwrap();
        let n = Rc::get_mut(&mut binding).unwrap();
        n.next = Some(Rc::new(SinglyLinkListNode {val: val, next: None }));
        self.length += 1;
    }
}

fn main() {
    let mut list = SinglyLinkList::create_singly_link_list(5);
    list.insert_val(3);

    println!("the values of head: {}", list.head.val);
    match list.head.next.as_ref() {
        None => println!("You fucked up insert the first node dumbass!!!!"),
        Some(n) => println!("the values of next: {}", n.val)
    }
}
