#include "armstrong_numbers.h"
#include <math.h>

bool is_armstrong_number(int candidate) 
{
  if (candidate == 0) return true;
  int factor = (int) ceil(log10(candidate));
  int sum = 0;
  for (int i = 0; i < factor; i++) {
    sum += pow((int)(candidate / pow(10, i)) % 10, factor);
  }
  return sum == candidate;
}
