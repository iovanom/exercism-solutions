#include <stdio.h>

#include "grains.h"

unsigned long long square(int nr) {
    if (nr < 1 || nr > 64) {
        return 0;
    }
    return 1ull << (nr - 1);
}

unsigned long long total() {
    return ~0ull;
}
