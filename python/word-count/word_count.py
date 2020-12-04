import re
from collections import defaultdict
from functools import reduce

strip_re = re.compile('^[^a-zA-Z0-9]+|[^a-zA-Z0-9]+$')
split_re = re.compile("[^a-zA-Z0-9']")

def count_words(sentence):
    def _calc(m, w):
        print(m)
        m[w] += 1
        return m

    return dict(reduce(_calc, filter(lambda w: w, map(
                    lambda w: strip_re.sub('', w),
                    split_re.split(sentence.lower())
                )), defaultdict(int)))
