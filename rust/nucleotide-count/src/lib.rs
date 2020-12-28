use std::collections::HashMap;

fn is_valid(nucleotide: char) -> bool {
    match nucleotide {
        'A' | 'C' | 'G' | 'T' => true,
        _ => false,
    }
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !is_valid(nucleotide) { return Err(nucleotide) }
    let mut count: usize = 0;
    for n in dna.chars() {
        if !is_valid(n) { return Err(n) }
        if nucleotide == n {
            count += 1;
        }
    }
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    // Create default nucleotide map counts
    let mut nucleotide_map: HashMap<char, usize> = "ACGT".chars()
        .map(|c| (c, 0))
        .collect();
    for n in dna.chars() {
        if !is_valid(n) { return Err(n) }
        nucleotide_map.entry(n).and_modify(|e| *e += 1);
    }
    Ok(nucleotide_map)
}
