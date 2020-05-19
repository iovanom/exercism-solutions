#include "gigasecond.h"

static const time_t gigasecond = 1e9;

time_t gigasecond_after(time_t after) {
    return after + gigasecond;
}
