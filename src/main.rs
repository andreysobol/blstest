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

    let keypair2 = Keypair::<Bls12>::generate(&mut rng);
    let message2 = "loooooooooooooooooooong message";
    let signature2 = keypair2.sign(&message2.as_bytes());

    let mut inputs = Vec::with_capacity(2);
    let mut signatures = Vec::with_capacity(2);

    inputs.push((keypair.public, message));
    inputs.push((keypair2.public, message2));
    signatures.push(sig);
    signatures.push(signature2);

    let asig = AggregateSignature::from_signatures(&signatures);
    assert_eq!(
        asig.verify(&inputs
            .iter()
            .map(|&(ref pk, ref m)| (pk, m.as_bytes()))
            .collect()),
        true
    );
}
