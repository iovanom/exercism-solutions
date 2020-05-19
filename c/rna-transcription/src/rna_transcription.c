#include <string.h>
#include <stdlib.h>

#include "rna_transcription.h"

static char transcribe_nucleotide(char nuc) {
    switch(nuc) {
        case 'G': return 'C';
        case 'C': return 'G';
        case 'T': return 'A';
        case 'A': return 'U';
        default: return '\0';
    }
}

char *to_rna(const char *dna) {
    if (dna == NULL) {
        return NULL;
    }
    int len = strlen(dna);
    char *rna = malloc(len + 1);
    for (int i = 0; i < len; i++) {
        // The nucleoid is not recognized, free rna and return NULL
        if (!(rna[i] = transcribe_nucleotide(dna[i]))) {
            free(rna);
            return NULL;
        }
    }
    rna[len] = '\0';
    return rna;
}
