use std::collections::{VecDeque, LinkedList};

fn main() {
    let mut llisti = LinkedList::from([1,2,3,4,5]);
    llisti.push_front(10);
    llisti.push_back(20);

    for l in &mut llisti {
        *l *= 2;
    }

    println!("fremst: {:?}", llisti.front());
    println!("afstast: {:?}", llisti.back());

    println!("{:?}", llisti);


    let mut listi: VecDeque<u8> = VecDeque::from([1,2,3,4]);
    listi.push_back(10u8);
    listi.push_front(5);

    for l in &listi {
        println!("{}", l);
    }

    for l in &mut listi {
        *l += 10;
    }

    println!("fremsta stakið: {:?}", listi.front());
    println!("aftasta stakið: {:?}", listi.back());

    println!("{:?}", listi);
}
