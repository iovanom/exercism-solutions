#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>
#include <stdbool.h>

#include "./acronym.h"

static int nr_of_words(const char *phrase);
static bool is_delimiter(char c);

char *abbreviate(const char *phrase) {
    if (phrase == NULL || phrase[0] == '\0') {
        return NULL;
    }
    int i = 0;
    int len = nr_of_words(phrase);
    char *abbrev = (char*)malloc(sizeof(char) * len + 1); 
    char *point = abbrev;
    while(phrase[i] != '\0') {
        if (isalnum(phrase[i]) && (i == 0 || is_delimiter(phrase[i-1]))) {
            *point++ = toupper(phrase[i]);
        }
        i++;
    }
    *point = '\0';
    return abbrev;
}

int nr_of_words(const char *phrase) {
    int i = 0;
    int count = 0;    
    while(phrase[i] != '\0') {
        if (is_delimiter(phrase[i++])) {
            count++;
        }
    }
    return count;
}

bool is_delimiter(char c) {
    return isspace(c) || c == '-';
}
