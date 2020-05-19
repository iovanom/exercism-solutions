#include <stdlib.h>
#include <string.h>
#include <stdio.h>

#include "matching_brackets.h"

typedef struct bracket_node {
    char bracket;
    struct bracket_node *prev;
} bracket_node_t;

static void clear_memory(bracket_node_t *head) {
    bracket_node_t *node;
    while(head != NULL) {
        node = head;
        head = head->prev;
        free(node);
    }
}

static inline bool is_bracket(char c) {
    switch(c) {
        case '[':
        case ']':
        case '{':
        case '}':
        case '(':
        case ')': return true;
        default: return false;
    }
}

static bracket_node_t *create_node(char bracket, bracket_node_t *head) {
    bracket_node_t *node = NULL;
    bool reduce_node = false;
    printf("%c\n", bracket);
    switch(bracket) {
        case '[': 
        case '{':
        case '(':
            node = (bracket_node_t*)malloc(sizeof(bracket_node_t));
            node->prev = head;
            node->bracket = bracket;
            break;
        case ']':
          if (head != NULL && head->bracket == '[') {
              reduce_node = true;
              break;
          }
          __attribute__((fallthrough));
        case '}':
          if (head != NULL && head->bracket == '{') {
              printf("%c\n", head->bracket);
              reduce_node = true;
              break;
          } 
          __attribute__((fallthrough));
        case ')':
          if(head != NULL && head->bracket == '(') {
              reduce_node = true;
              break;
          }
          __attribute__((fallthrough));
        default: 
          // the error occurred
        node = (bracket_node_t*)malloc(sizeof(bracket_node_t));
        node->bracket = 'e';
    }
    if (reduce_node) {
        node = head->prev;
        free(head);
    }
    return node;
}

bool is_paired(const char *input) {
    if (input == NULL) {
        return true;
    }
    printf("%s\n", input);
    int len = strlen(input);
    bracket_node_t *head = NULL;
    for (int i = 0; i < len; i++) {
        printf("%d\n", i);
        if (is_bracket(input[i])) {
            head = create_node(input[i], head);
            printf("%c\n", head != NULL ? head->bracket : 'n');
            if (head != NULL && head->bracket == 'e') {
                clear_memory(head);
                return false;
            }
        }
    }
    if (head != NULL) {
        clear_memory(head);
        return false;
    }
    return true;
}
