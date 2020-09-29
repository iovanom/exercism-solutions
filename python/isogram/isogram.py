def is_isogram(string: str) -> bool:
    string = string.lower()
    for i, char in zip(range(len(string)), string):
        if char.strip().isalpha() and char in string[i+1:]:
            return False
    return True

