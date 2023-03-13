/// The Vigenere cipher is the following
///
///  | |A|B|C|D|E|F|G|H|I|J|K|L|M|N|O|P|Q|R|S|T|U|V|W|X|Y|Z
/// _____________________________________________________
/// A| |A|B|C|D|E|F|G|H|I|J|K|L|M|N|O|P|Q|R|S|T|U|V|W|X|Y|Z
/// B| |B|C|D|E|F|G|H|I|J|K|L|M|N|O|P|Q|R|S|T|U|V|W|X|Y|Z|A
/// C| |C|D|E|F|G|H|I|J|K|L|M|N|O|P|Q|R|S|T|U|V|W|X|Y|Z|A|B
/// D| |D|E|F|G|H|I|J|K|L|M|N|O|P|Q|R|S|T|U|V|W|X|Y|Z|A|B|C
/// E| |E|F|G|H|I|J|K|L|M|N|O|P|Q|R|S|T|U|V|W|X|Y|Z|A|B|C|D
/// F| |F|G|H|I|J|K|L|M|N|O|P|Q|R|S|T|U|V|W|X|Y|Z|A|B|C|D|E
/// G| |G|H|I|J|K|L|M|N|O|P|Q|R|S|T|U|V|W|X|Y|Z|A|B|C|D|E|F
/// H| |H|I|J|K|L|M|N|O|P|Q|R|S|T|U|V|W|X|Y|Z|A|B|C|D|E|F|G
/// I| |I|J|K|L|M|N|O|P|Q|R|S|T|U|V|W|X|Y|Z|A|B|C|D|E|F|G|H
/// J| |J|K|L|M|N|O|P|Q|R|S|T|U|V|W|X|Y|Z|A|B|C|D|E|F|G|H|I
/// K| |K|L|M|N|O|P|Q|R|S|T|U|V|W|X|Y|Z|A|B|C|D|E|F|G|H|I|J
/// L| |L|M|N|O|P|Q|R|S|T|U|V|W|X|Y|Z|A|B|C|D|E|F|G|H|I|J|K
/// M| |M|N|O|P|Q|R|S|T|U|V|W|X|Y|Z|A|B|C|D|E|F|G|H|I|J|K|L
/// N| |N|O|P|Q|R|S|T|U|V|W|X|Y|Z|A|B|C|D|E|F|G|H|I|J|K|L|M
/// O| |O|P|Q|R|S|T|U|V|W|X|Y|Z|A|B|C|D|E|F|G|H|I|J|K|L|M|N
/// P| |P|Q|R|S|T|U|V|W|X|Y|Z|A|B|C|D|E|F|G|H|I|J|K|L|M|N|O
/// Q| |Q|R|S|T|U|V|W|X|Y|Z|A|B|C|D|E|F|G|H|I|J|K|L|M|N|O|P
/// R| |R|S|T|U|V|W|X|Y|Z|A|B|C|D|E|F|G|H|I|J|K|L|M|N|O|P|Q
/// S| |S|T|U|V|W|X|Y|Z|A|B|C|D|E|F|G|H|I|J|K|L|M|N|O|P|Q|R
/// T| |T|U|V|W|X|Y|Z|A|B|C|D|E|F|G|H|I|J|K|L|M|N|O|P|Q|R|S
/// U| |U|V|W|X|Y|Z|A|B|C|D|E|F|G|H|I|J|K|L|M|N|O|P|Q|R|S|T
/// V| |V|W|X|Y|Z|A|B|C|D|E|F|G|H|I|J|K|L|M|N|O|P|Q|R|S|T|U
/// W| |W|X|Y|Z|A|B|C|D|E|F|G|H|I|J|K|L|M|N|O|P|Q|R|S|T|U|V
/// X| |X|Y|Z|A|B|C|D|E|F|G|H|I|J|K|L|M|N|O|P|Q|R|S|T|U|V|W
/// Y| |Y|Z|A|B|C|D|E|F|G|H|I|J|K|L|M|N|O|P|Q|R|S|T|U|V|W|X
/// Z| |Z|A|B|C|D|E|F|G|H|I|J|K|L|M|N|O|P|Q|R|S|T|U|V|W|X|Y
///
///
/// So for,
///  
/// Plaintext message:
///     ATTACKATDAWN
///
/// Key:
///     LEMON
///
/// Keystream:
///     LEMONLEMONLE
///
/// Ciphertext:
///     LXFOPVEFRNHR
///
/// Tim's implementation has no allocations
///
/// I was using HashMap to represent the cipher table

mod vigenere {
    const ALPHABET: [u8; 26] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const A: u8 = b'A';
    const Z: u8 = b'Z';
    const WRAP: u8 = 26; // ALPHABET.len() as u8

    fn clean_input(input: &str) -> impl Iterator<Item = u8> + '_ {
        input.bytes().filter_map(|x| match x {
            A..=Z => Some(x),
            b'a'..=b'z' => Some(x - (b'a' - A)),
            _ => None,
        })
    }

    pub fn encrypt(plaintext: &str, key: &str) -> String {
        let mut key_iter = key
            .bytes()
            .map(|b| b - A) // bump into ascii range
            .cycle(); // I was LOOKING FOR THIS METHOD

        let ciphertext = clean_input(plaintext)
            .map(|x| {
                let letter_in_key = key_iter.next().unwrap();

                // TODO: alter this line to encrypt plaintext
                //v value between 0 and 26           v
                ((x + A) - letter_in_key) % WRAP + A
                //                                        ^ pushes back into ascii range for uppercase letters
            })
            .collect();

        String::from_utf8(ciphertext).unwrap()
    }

    pub fn decrypt(ciphertext: &str, key: &str) -> String {
        let mut key_iter = key
            .bytes()
            .map(|b| b - A) // bump into ascii range
            .cycle(); // I was LOOKING FOR THIS METHOD

        let ciphertext = clean_input(ciphertext)
            .map(|x| {
                let letter_in_key = key_iter.next().unwrap();
                //v value between 0 and 26           v
                ((x + WRAP - A) - letter_in_key) % WRAP + A
                //                                        ^ pushes back into ascii range for uppercase letters
            })
            .collect();

        String::from_utf8(ciphertext).unwrap()
    }
}

fn main() {
    let key = "WHYRUST";
    let ciphertext = "
    PVCDJG
    PAYCMY
    JR KUC
    ";
    let plaintext = vigenere::decrypt(&ciphertext, key);

    println!("{}", plaintext);

    let ciphertext = vigenere::encrypt(&ciphertext, key);

    println!("{}", ciphertext);
}
