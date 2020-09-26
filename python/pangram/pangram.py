ALPHABET_LENGTH = ord('z') - ord('a') + 1


def is_pangram(sentence: str) -> bool:
    """ Check if provided sentence is pangram """
    return len(set([c for c in sentence.lower() if c.strip().isalpha()])) == ALPHABET_LENGTH
