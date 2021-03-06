CONFIGS = (
    ('AEIOULNRST', 1),
    ('DG', 2),
    ('BCMP', 3),
    ('FHVWY', 4),
    ('K', 5),
    ('JX', 8),
    ('QZ', 10),
)

SCORES = {c: s for chars, s in CONFIGS for c in chars} 

def score(word):
    return sum(map(lambda c: SCORES[c], word.upper()))
