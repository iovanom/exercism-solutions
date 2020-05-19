#include <ctype.h>
#include <string.h>

#include "word_count.h"

static word_count_word_t create_new_word_count_word(const char *word) {
    word_count_word_t wcw;
    memset(wcw.text, 0, MAX_WORD_LENGTH+1);
    int j = 0;
    for (int i = 0; word[i] != '\0'; i++) {
        if (i == 0 && word[i] == '\'') {
            continue;
        }
        wcw.text[j++] = word[i];
    }
    if (wcw.text[j-1] == '\'') {
        wcw.text[j-1] = '\0';
    }
    wcw.count = 1;
    return wcw;
}

static int strcicmp(const char *w1, const char *w2) {
    for (int i = 0;; i++) {
        if (tolower(w1[i]) != tolower(w2[i])) {
            return -1;
        }

        if (w1[i] == '\0') {
            return 0;
        }
    }
}

static int find_word(const char *word, word_count_word_t *words, int count) {
    for(int i = 0; i < count; i++) {
        if (strcicmp(word, words[i].text) == 0) {
            return i;
        }
    }
    return -1;
}

int word_count(const char *input_text, word_count_word_t *words) {
    int count = 0;
    int char_count = 0;
    int word_index = -1;
    char word[MAX_WORD_LENGTH+1];

    // clear the words array
    memset(words, 0, sizeof(word_count_word_t) * MAX_WORDS);

    int text_len = strlen(input_text);
    for (int i = 0; i <= text_len; i++) {
        if (isalnum(input_text[i]) || input_text[i] == '\'') {
            if (char_count == MAX_WORD_LENGTH) {
                return EXCESSIVE_LENGTH_WORD;
            }
            word[char_count++] = tolower(input_text[i]);
        } else if (char_count) {
            // we have a word
            word[char_count] = '\0';
            char_count = 0;
            word_index = find_word(word, words, count);
            if (word_index != -1) {
                words[word_index].count++;
            } else if (count == MAX_WORDS) {
                return EXCESSIVE_NUMBER_OF_WORDS;
            } else {
                words[count++] = create_new_word_count_word(word);
            }
        }
    }
    return count;
}
