package strand

var DNAtoRNA = map[byte]byte{
	'G': 'C',
	'C': 'G',
	'T': 'A',
	'A': 'U',
}

func ToRNA(dna string) string {
	var rna []byte
	for _, b := range []byte(dna) {
		rna = append(rna, DNAtoRNA[b])
	}
	return string(rna)
}
