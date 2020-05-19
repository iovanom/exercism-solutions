#include <string.h>
#include <ctype.h>

#include "pangram.h"

#define chars_len ('z' - 'a' + 1)

bool is_pangram(const char *sentence) {
    bool chars[chars_len] = {0};
    int len;
    if (sentence == NULL || (len = strlen(sentence)) == 0) {
        return false;
    }
    for (int i = 0; i < len; i++) {
        if (isalpha(sentence[i])) {
            chars[tolower(sentence[i]) - 'a'] = true;
        }
    }
    for (int i = 0; i < chars_len; i++) {
        if (!chars[i]) {
            return false;
        }
    } 
    return true;
}
