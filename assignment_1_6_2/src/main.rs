fn main() {
    // "glovars" / program tilstand
    // Nu32 -> værdi N af typen unsigned int 32
    let belob = 100u32;

    // mut = mutable = "assignable"
    let (mut en, mut to, mut fem, mut ti, mut tyve) = (0u32, 0u32, 0u32, 0u32, 0u32);

    // @ pre / Q
    assert_ne!(belob, 0);

    // implementation / S
    {
        let mut b = belob;

        while b >= 20 {
            tyve += 1;
            b -= 20;
        }

        while b >= 10 {
            ti += 1;
            b -= 10;
        }

        while b >= 5 {
            fem += 1;
            b -= 5;
        }

        while b >= 2 {
            to += 1;
            b -= 2;
        }

        while b >= 1 {
            en += 1;
            b -= 1;
        }
    }

    // @ post / R
    assert_eq!(en + (2 * to) + (5 * fem) + (10 * ti) + (20 * tyve),  belob);

    // visual output, not part of assignment
    println!("1: {}, 2: {}, 5: {}, 10: {}, 20: {}", en, to, fem, ti, tyve);
}
