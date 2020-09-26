ISBN_LEN = 10

def is_valid(isbn):
    """ Validate the ISBN-10 number
    The ISBN-10 format is 9 digits (0 to 9) plus one check character 
    (either a digit or an X only). In the case the check character is an X,
    this represents the value '10'. These may be communicated with or without 
    hyphens, and can be checked for their validity by the following formula:
        `(x1 * 10 + x2 * 9 + x3 * 8 + x4 * 7 + x5 * 6 + x6 * 5 + x7 * 4 + x8 *
        3 + x9 * 2 + x10 * 1) mod 11 == 0`
    """
    isbn = isbn.replace('-', '')
    if len(isbn) != ISBN_LEN:
        return False
    try:
        return not sum(
            [(ISBN_LEN-i) * (10 if n == 'X' and i == ISBN_LEN-1 else int(n)) for (i, n) in
             zip(range(ISBN_LEN), isbn)]) % 11
    except ValueError:
        return False
