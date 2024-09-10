
//fn tveir_eins(listinn: &Vec<i32>) -> bool {
fn tveir_eins(listinn: &[i32]) -> bool {
    // fara í gegnum listann með Windows og 
    // skila true ef einhversstaðar eru tvær
    // eins tölur í röð
}

fn main() {
    let mut tomur_listi: Vec<u32> = Vec::new();
    let mut listi = vec![1,2,3,4,5,6];
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

    for l in listi.windows(2) {
        println!("{:?}", l);
    }


    println!("{:?}", listi);
}
