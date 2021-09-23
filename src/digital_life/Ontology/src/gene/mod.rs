use easy_hasher::easy_hasher::keccak256;

pub fn generate_random_dna(_str: String) -> Vec<u32> {
    let dna_modulus: u32 = 3;
    let rand = keccak256(&_str);
    let gene = rand.to_hex_string().parse::<u32>().unwrap_or(100) % dna_modulus;

    vec![gene]
}

pub struct Details {
    // first 2 digits make up the head. We have 7 possible heads, so % 7
    // to get a number 0 - 6, then add 1 to make it 1 - 7. Then we have 7
    // image files named "head1.png" through "head7.png" we load based on
    // this number:
    head_choice: u32,
    // 2nd 2 digits make up the eyes, 11 variations:
    eye_choice: u32,
    // 6 variations of shirts:
    shirt_choice: u32,
    // last 6 digits control color. Updated using CSS filter: hue-rotate
    // which has 360 degrees:
    skin_color_choice: u32,
    eye_color_choice: u32,
    clothes_color_choice: u32,
}

pub fn build_from_gene(dna: String) -> Details{
    // pad DNA with leading zeroes if it's less than 16 characters
    let mut dna_str = dna;
    while dna_str.len() < 16 {
        dna_str = String::from("0") + &dna_str;
    }

    let detail= Details {
        head_choice: &dna_str[0..2].parse::<u32>().unwrap() % 7 + 1,
        eye_choice: &dna_str[2..4].parse::<u32>().unwrap() % 11 + 1,
        shirt_choice: &dna_str[4..6].parse::<u32>().unwrap() % 6 + 1,
        skin_color_choice: &dna_str[6..8].parse::<u32>().unwrap() / 100 * 360,
        eye_color_choice: &dna_str[8..10].parse::<u32>().unwrap() / 100 * 360,
        clothes_color_choice: &dna_str[10..12].parse::<u32>().unwrap() / 100 * 360,
    };

    detail
}
