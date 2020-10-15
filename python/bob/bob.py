ANSWERS = (
    ((lambda t: t.isupper() and t.endswith('?')),
     "Calm down, I know what I'm doing!"),
    (lambda t: t.isupper(), "Whoa, chill out!"),
    (lambda t: t.endswith('?'), "Sure."),
    (lambda t: bool(t), "Whatever."),
    (lambda t: not bool(t), "Fine. Be that way!")
)

def response(hey_bob):
    for (check, answer) in ANSWERS:
        if check(hey_bob.strip()):
            return answer
