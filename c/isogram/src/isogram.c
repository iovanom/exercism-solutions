#include <ctype.h>
#include <stddef.h>
#include <string.h>

#include "isogram.h"

bool is_isogram(const char phrase[]) {
    if (phrase == NULL) 
        return false;
    int len = strlen(phrase);
    int letter_mask = 0;
    for (int i = 0; i < len; i++) {
        if (!isalpha(phrase[i])) {
            continue;
        }
        int letter_bit = 1 << (tolower(phrase[i]) - 'a');
        if (letter_mask & letter_bit) {
            return false;
        }
        letter_mask = letter_bit | letter_mask;
    }
    return true;
}
