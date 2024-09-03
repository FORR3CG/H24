fn main() {
    let mut k = 5;
    while k > 0 {
        dbg!(k);
        k -= 1;
    }

    for i in (1..=5).rev().step_by(2) { // range(5, 1, -2)
        dbg!(i);
    }

    let fylki = [1,2,3,4,5];

    for stak in fylki {
        dbg!(stak);
    }

    let mut teljari = 5;
    'ytri: loop {
        let mut nidurteljari = 10;
        'innri: loop {
            teljari -= 1;
            nidurteljari -= 1;
            if teljari == 0 {
                break 'ytri;
            }
        }
    }

    let mut k = 5;
    let mut summa = 0;
    let fimm_k = loop {
        summa += k;
        k -= 1; 
        if k == 0 {
            break summa
        }
    };
    dbg!(fimm_k);
    let u = 10;
    let k = if u == 10 { 10 } else { 20 };
    // k = 5 if u == 10 else 20
    // const k = (1 < 2) ? 20 : 30

}

/*
listi = [1,2,3,4,5], refcount fyrir listi: 0
listi2 = listi

...
...

print(listi2)
..
..


*/
