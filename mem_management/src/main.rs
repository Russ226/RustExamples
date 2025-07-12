use std::{cell::RefCell, clone, rc::{Rc, Weak}};

#[derive(Debug, Clone)]
struct Person{
    age:u8,
    name: String
}

impl Person {
    fn set_age(&mut self, new_age: u8){
        self.age = new_age;
    }

     fn set_name(&mut self, new_name: String){
        self.name = new_name;
    }
}

fn main() {
    let person = Rc::new(RefCell::new(Person {age: 10, name: String::from("test")}));

    //person.set_age(8);
    
    let p = Rc::clone(&person);
    //p.name = String::from("changed");
    let p2 = Rc::clone(&p);
    person.borrow_mut().set_age(8);
    println!("the address of p is {:p} \n {:?}", &p, p);
    println!("the address of p2 is {:p} \n {:?}", &p2, p2);
    println!("the address of person is {:p} \n {:?}", &person, person);


    let weak_person = Weak::new(Person {age: 2, name: String::from("weak")});
}
