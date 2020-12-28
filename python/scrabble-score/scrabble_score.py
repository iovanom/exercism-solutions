configs = (
    ('AEIOULNRST', 1),
    ('DG', 2),
    ('BCMP', 3),
    ('FHVWY', 4),
    ('K', 5),
    ('JX', 8),
    ('QZ', 10),
)

scores = { c: s for (chars, s) in configs for c in chars } 

def score(word):
    return sum(map(lambda c: scores[c], word.upper()))
