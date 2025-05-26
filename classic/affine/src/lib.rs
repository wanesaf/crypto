use num::integer;
pub struct AffineCoefs {
    pub a: u32,
    pub b: u32, 
}
#[derive(Debug)]
pub struct PlainAndCipher {
    pub texte_clair: String,
    pub texte_chiffre: String,
    pub texte_dechiffre: String,
}

impl AffineCoefs {
    pub fn new(a: u32, b: u32) -> AffineCoefs {
        if integer::gcd(a % 26, 26) != 1 {
            panic!("a must be prime with 26 (to get an inverse)");
        }
        AffineCoefs { a, b }
    }
}

impl PlainAndCipher {
    pub fn new(input: String) -> PlainAndCipher {
        PlainAndCipher {
            texte_clair: input,
            texte_chiffre: String::new(),
            texte_dechiffre: String::new(),
        }
    }
    pub fn chiffre(&mut self, affine: &AffineCoefs) {
        for c in self.texte_clair.chars() {
            let uc = c.to_ascii_uppercase();
            if uc.is_ascii_alphabetic() {
                // verifier est ce que ascii tae lettre
                let plain = uc as u8 - b'A'; // b mean bytes , tsema rana nconvertiw lplain l u8
                let result = (affine.a * plain as u32 + affine.b) % 26; //appliquer formule
                self.texte_chiffre.push((result as u8 + b'A') as char); // rana ndiro mod 26 ez cast from u32 to u8 without errors
            } else {
                self.texte_chiffre.push(c); // nkhaliw la lettre kima rahi ida makanetch lettre zaema ra9m
            }
        }
    }
    pub fn dechiffre(&mut self, affine: &AffineCoefs) {
        let a_inv = modinv(affine.a,z26x());
        for c in self.texte_chiffre.chars() {
            let uc = c.to_ascii_uppercase();
            if uc.is_ascii_alphabetic() {
                let chiffre = uc as u8 - b'A';
                let result = ((chiffre as u32 - affine.b) * a_inv) % 26;
                self.texte_dechiffre.push((result as u8 + b'A' ) as char) ;
            }
            else {
                self.texte_dechiffre.push(c);
            }
        }
    }
}

pub fn z26x() -> Vec<u8> {
    //(Z/26Z)*
    let mut z26x = Vec::new();

    for i in 1..26 {
        if integer::gcd(i, 26) == 1 {
            z26x.push(i);
        }
    }

    z26x
}

pub fn modinv(a: u32, z26x: Vec<u8>) -> u32 {
    // nverfiyi berk les elements tae (z/26Z)*
    for k in z26x {
        if a * (k as u32) % 26 == 1 {
            return k as u32;
        }
    }
    return 0; // hadi dertha berk haka , jamais rah tred 0 pisk verifyi melewel bli input a rah ykon prime mea 26 (tsma inverse dyalo yexsiti f z/26Z)
}
