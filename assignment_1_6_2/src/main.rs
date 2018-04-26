fn main() {
    // "glovars" / program tilstand
    // Nu32 -> værdi N af typen unsigned int 32
    let belob = 0u32;

    // mut = mutable = "assignable"
    let (mut en, mut to, mut fem, mut ti, mut tyve) = (0u32, 0u32, 0u32, 0u32, 0u32);

    // @ pre / Q
    //assert_eq!(belob >= 0, true);
    // above is useless due to existing type limits

    // implementation / S
    {
        let mut b = belob;

        while b > 0 {
            if b >= 20 {
                tyve += 1;
                b -= 20;
            } else if b >= 10 {
                ti += 1;
                b -= 10;
            } else if b >= 5 {
                fem += 1;
                b -= 5;
            } else if b >= 2 {
                to += 1;
                b -= 2;
            } else if b >= 1 {
                en += 1;
                b -= 1;
            }
        }
    }

    // @ post / R
    assert_eq!(en + (2 * to) + (5 * fem) + (10 * ti) + (20 * tyve),  belob);

    // visual output, not part of assignment
    println!("1: {}, 2: {}, 5: {}, 10: {}, 20: {}", en, to, fem, ti, tyve);
}
