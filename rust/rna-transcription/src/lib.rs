use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Dna {
    stand: String,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    stand: String,
}

impl Dna {
    const NUCLEOTIDES: &'static str = "GCTA";

    pub fn new(nucleo_str: &str) -> Result<Self, usize> {
        let stand = nucleo_str
            .chars()
            .enumerate()
            .map(|(i, n)| {
                if !Self::NUCLEOTIDES.contains(n) {
                    Err(i)
                } else {
                    Ok(n)
                }
            })
            .collect::<Result<String, usize>>()?;
        Ok(Self { stand })
    }

    pub fn into_rna(self) -> Rna {
        let map: HashMap<char, char> = Dna::NUCLEOTIDES
            .chars()
            .zip(Rna::NUCLEOTIDES.chars())
            .collect();
        let stand = self
            .stand
            .chars()
            .filter_map(|n| map.get(&n))
            .collect::<String>();
        Rna { stand }
    }
}

impl Rna {
    const NUCLEOTIDES: &'static str = "CGAU";

    pub fn new(nucleo_str: &str) -> Result<Self, usize> {
        let stand = nucleo_str
            .chars()
            .enumerate()
            .map(|(i, n)| {
                if !Self::NUCLEOTIDES.contains(n) {
                    Err(i)
                } else {
                    Ok(n)
                }
            })
            .collect::<Result<String, usize>>()?;
        Ok(Self { stand })
    }
}
