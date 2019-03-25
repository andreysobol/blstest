extern crate bls;
extern crate rand;

fn main() {
    use bls::{AggregateSignature, Keypair};
    use pairing::bls12_381::Bls12;
    use rand::{SeedableRng, XorShiftRng};

    let mut rng = XorShiftRng::from_seed([0xbc4f6d44, 0xd62f276c, 0xb963afd0, 0x5455863d]);

    let keypair = Keypair::<Bls12>::generate(&mut rng);
    let message = "Looooooooooooooooooooooooong";
    let sig = keypair.sign(&message.as_bytes());
    assert_eq!(keypair.verify(&message.as_bytes(), &sig), true);
}
