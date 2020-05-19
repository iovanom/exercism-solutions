#include <ctype.h>
#include <string.h>

#include "hamming.h"

int compute(const char *dna1, const char *dna2) {
    int len;
    int count = 0;
    if (dna1 == NULL || dna2 == NULL || (len = strlen(dna1)) != (int)strlen(dna2)) {
        return ERROR_RESPONSE;
    }
    for (int i = 0; i < len; i++) {
        if (toupper(dna1[i]) != toupper(dna2[i])) {
            count++;
        }
    }
    return count;
}
