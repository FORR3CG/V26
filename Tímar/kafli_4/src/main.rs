fn main() {
    let k = 10;
    println!("{:p}", &k);
    let k2 = 10;
    println!("{:p}", &k2);
    {
        let j = Box::new(25);
        println!("{:p}", j);
    }
        
    let j2 = Box::new(27);
    println!("{:p}", j2);
}
/*
    c málin
    malloc -> setur hluti á heap
    free -> hreinsar af heap

    python
    garbage collection - reference counting
 */
