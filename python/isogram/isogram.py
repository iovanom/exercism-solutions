def is_isogram(string: str) -> bool:
    string = string.lower()
    for i, char in enumerate(string):
        if char.strip().isalpha() and char in string[i+1:]:
            return False
    return True

