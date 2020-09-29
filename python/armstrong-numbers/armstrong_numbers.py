import math

def is_armstrong_number(number: int) -> bool:
    if number == 0:
        return True
    factor = int(math.log(number, 10)) + 1
    return number == sum(int(number/(10**i)%10)**factor for i in range(factor))


"""
First solution with str
```
def is_armstrong_number(number):
    str_number = str(number)
    factor = len(str_number)
    return number == sum(int(n)**factor for n in str_number)
```
"""
