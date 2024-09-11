use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::cmp::Reverse;

fn main() {

    let mut bh = BinaryHeap::new();

    bh.push(Reverse(5));
    bh.push(Reverse(1));
    bh.push(Reverse(3));
    bh.push(Reverse(2));
    bh.push(Reverse(6));
    bh.push(Reverse(8));
    bh.push(Reverse(7));

    for b in &bh {
        println!("{}", b.0);
    }

    for _ in 0..3 {
        println!("{:?}", bh.pop());
    }

    println!("{:?}", bh);

    //let mut hs = HashSet::new();
    let mut hs = BTreeSet::new();
    hs.insert("Ísland");
    hs.insert("Danmörk");
    hs.insert("Svíþjóð");
    hs.insert("Ísland");

    println!("{:?}", hs);


    let mut hm: HashMap<&str, i32> = HashMap::new();
    //let mut hm = BTreeMap::new();

    hm.insert("Ísland", 100);
    hm.insert("Danmörk", 80);
    hm.insert("Noregur", 50);
    hm.insert("Svíþjóð", 70);

    dbg!(hm.get("Ísland"));
    dbg!(hm.get_key_value("Ísland"));
    dbg!(hm.contains_key("Finnland"));
    
    dbg!(hm.entry("Finnland").or_insert(40));
    dbg!(hm.entry("Ísland").or_insert(999));

    //println!("cap: {}, len: {}", hm.capacity(), hm.len());

    println!("{:#?}", hm);
    for h in &mut hm {
        *h.1 += 10;
    }

    for h in &hm {
        println!("allt: {:?}, key: {}, value: {}", h, h.0, h.1);
    }


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
