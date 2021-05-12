const VALID_DNA_NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];
const VALID_RNA_NUCLEOTIDES: [char; 4] = ['U', 'G', 'C', 'A'];

#[derive(Debug, PartialEq)]
pub struct Dna(String);

#[derive(Debug, PartialEq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (i, n) in dna.chars().enumerate() {
            if !VALID_DNA_NUCLEOTIDES.contains(&n) {
                return Err(i);
            }
        }
        Ok(Self(dna.to_string()))
    }

    pub fn into_rna(self) -> Rna {
        let s = self
            .0
            .chars()
            .map(|c| {
                let index = VALID_DNA_NUCLEOTIDES.iter().position(|&p| p == c).unwrap();
                VALID_RNA_NUCLEOTIDES[index]
            })
            .collect();

        Rna(s)
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (i, n) in rna.chars().enumerate() {
            if !VALID_RNA_NUCLEOTIDES.contains(&n) {
                return Err(i);
            }
        }
        Ok(Self(rna.to_string()))
    }
}
