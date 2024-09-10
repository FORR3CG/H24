
//fn tveir_eins(listinn: &Vec<i32>) -> bool {
fn tveir_eins(listinn: &[i32]) -> Option<i32> {
    // fara í gegnum listann með Windows og 
    // skila true ef einhversstaðar eru tvær
    // eins tölur í röð
    for l in listinn.windows(2) {
        if l.first() == l.last() {
            //return Some(*l.first().unwrap());
            match l.first() {
                Some(tala) => return Some(*tala),
                None => continue,
            }
        }
    }
    None
}

fn main() {
    let mut tomur_listi: Vec<u32> = Vec::new();
    let mut listi = vec![1,2,3,4,5,6,7];
    listi[2] = 99;
    listi.get(2).replace(&55);
    println!("{:?}", tveir_eins(&listi));
    listi.push(7); // LIFO -> 
    let item = listi.pop();
    let fyrsta_stakid = listi.remove(0);
    listi.swap_remove(0); // [2,3,4,5,6]
    println!("stak nr. 0: {:?}", listi.get(100)); // notum get í stað []
    println!("stak nr. 0: {:?}", listi.first()); // notum first í stað get(0)
    println!("stak nr. 0: {:?}", listi.last()); // notum last til að fá síðasta stakið
    for l in &listi {
        println!("{}", l);
    }

    for l in &mut listi {
        *l += 10
    }

    for l in listi.chunks(3) {
        println!("{:?}", l);
    }
    println!("windows start");
    for l in listi.windows(4) {
        println!("{:?}", l);
    }
    println!("windows end");


    println!("{:?}", listi);
    listi.sort();
    println!("{:?}", listi);
}
