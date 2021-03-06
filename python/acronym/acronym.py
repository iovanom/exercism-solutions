import re

WORD_RE = re.compile("[a-zA-Z][a-zA-Z0-9]*'*[a-zA-Z0-9]+|[a-zA-Z]")

def abbreviate(words):
    return "".join(map(lambda w: w[0], WORD_RE.findall(words))).upper()
