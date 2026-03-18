fn main() {
    /* 
    listi = [1,2,3,4]
    listi2 = listi
    listi2[0] = 99
    print(listi)
    */
    // aðeins einn getur átt gögnin
    let mut s1: String = String::from("Tskoli");
    // margir geta fengið gögnin lánuð með lesréttindum (read only) (með &)
    //let s2 = &s1;
    //let s3 = &s1;
    // einn má fá lána mut (write)
    // ekki er hægt að blanda saman lánum með les og skrifréttindi
    //let mut s4: &mut String = &mut s1;
    // fall_sem_tekur_eignahald(s1);
    // fall_sem_faer_lanad(&s1);
    // s1 = fall_sem_tekur_eignahald_og_skilar(s1);
    // s1 = fall_sem_tekur_eignahald_breytir_skilar(s1);
    fall_sem_faer_lanad_mut(&mut s1);
    println!("{}", s1);
    //println!("{}", s3);
    
    /*
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
    */
}

fn fall_sem_faer_lanad_mut(texti: &mut String) {
    texti.push_str("TTT")
}

fn fall_sem_tekur_eignahald_breytir_skilar(mut texti: String) -> String {
    texti.push_str("TTT");
    texti
}

fn fall_sem_tekur_eignahald_og_skilar(texti: String) -> String {
    texti.to_uppercase()
}

fn fall_sem_faer_lanad(texti: &String) {
    println!("{}", texti.to_uppercase())
}

fn fall_sem_tekur_eignahald(texti: String) {
    println!("{}", texti.to_uppercase())
}
/*
    c málin
    malloc -> setur hluti á heap
    free -> hreinsar af heap

    python
    garbage collection - reference counting
 */
