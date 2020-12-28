import re
from collections import Counter

word_re = re.compile("[a-zA-Z0-9]+'*[a-zA-Z0-9]+|[a-zA-Z0-9]")

def count_words(sentence):
    return Counter(word_re.findall(sentence.lower()))
