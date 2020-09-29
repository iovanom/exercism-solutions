def distance(strand_a: str, strand_b: str):
    if len(strand_a) != len(strand_b):
        raise ValueError("Different lengths")
    return sum(a != b for a, b in zip(strand_a, strand_b))
