#include <stdio.h>
#include <string.h>

#include "beer_song.h"

static const char *format_string_common =
    "%d bottles of beer on the wall, %d bottles of beer.\n"
   "Take one down and pass it around, %d bottles of beer on the wall.\n";

static const char *format_string_for_two =
    "2 bottles of beer on the wall, 2 bottles of beer.\n"
   "Take one down and pass it around, 1 bottle of beer on the wall.\n";

static const char *format_string_for_one =
   "1 bottle of beer on the wall, 1 bottle of beer.\n"
   "Take it down and pass it around, no more bottles of beer on the wall.\n";

static const char *format_string_for_zero = 
   "No more bottles of beer on the wall, no more bottles of beer.\n"
   "Go to the store and buy some more, 99 bottles of beer on the wall.\n";

void verse(char *response, int botle_count) {
    if (botle_count > 2) {
        sprintf(response, format_string_common, botle_count,
                botle_count, botle_count-1);
    } else if (botle_count == 2) {
        sprintf(response, format_string_for_two);
    } else if (botle_count == 1) {
        sprintf(response, format_string_for_one);
    } else {
        sprintf(response, format_string_for_zero);
    }
}

void sing(char *response, int botle_count, int end_count) {
    // reset the string length to 0;
    response[0] = '\0';
    for (int i = botle_count; i >= end_count; i--) {
        printf("%d\n", i);
        verse((char*)(response + strlen(response)), i);
        // check if it's not last iteration
        if (i != end_count) {
            // add new line
            int len = strlen(response);
            response[len] = '\n';
            response[len+1] = '\0';
        }
    }
}
