package protein

import (
	"errors"
)

// ErrInvalidBase is error when the codon is not valid
var ErrInvalidBase = errors.New("ERROR")

// ErrStop is error for stop codons
var ErrStop = errors.New("STOP")

var codonMap = map[string]string{
	"AUG": "Methionine",
	"UUU": "Phenylalanine",
	"UUC": "Phenylalanine",
	"UUA": "Leucine",
	"UUG": "Leucine",
	"UCU": "Serine",
	"UCC": "Serine",
	"UCA": "Serine",
	"UCG": "Serine",
	"UAU": "Tyrosine",
	"UAC": "Tyrosine",
	"UGU": "Cysteine",
	"UGC": "Cysteine",
	"UGG": "Tryptophan",
	"UAA": "STOP",
	"UAG": "STOP",
	"UGA": "STOP",
}

// FromCodon function translate codon to protein
func FromCodon(codon string) (string, error) {
	protein, ok := codonMap[codon]
	if !ok {
		return "", ErrInvalidBase
	}
	if protein == "STOP" {
		return "", ErrStop
	}
	return protein, nil
}

// FromRNA function translate RNA sequences to proteins
func FromRNA(rna string) (proteins []string, err error) {
	for i := 3; i <= len(rna); i += 3 {
		protein, ok := codonMap[rna[i-3:i]]
		if !ok {
			return proteins, ErrInvalidBase
		}
		if protein == "STOP" {
			return proteins, nil
		}
		proteins = append(proteins, protein)
	}
	return proteins, nil
}
