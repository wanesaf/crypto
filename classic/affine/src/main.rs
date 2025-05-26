use affine::{PlainAndCipher, AffineCoefs};

fn main() {
    let mut x = PlainAndCipher::new(String::from("ECOLE")) ;
    let affine = AffineCoefs::new(17, 3);
    x.chiffre(&affine);
    x.dechiffre(&affine);
    println!("{:#?}",x);
    
}
