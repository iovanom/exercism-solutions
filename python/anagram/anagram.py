def find_anagrams(word, candidates):
    word_sort = sorted(word.lower())
    return [ candidate for candidate in candidates 
            if sorted(candidate.lower()) == word_sort 
            and word.lower() != candidate.lower() ]
