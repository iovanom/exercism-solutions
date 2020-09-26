def latest(scores):
    """ Return latest score """
    return scores[-1]


def personal_best(scores):
    """ Return the best score """
    return max(scores)


def personal_top_three(scores):
    """ Return three best scores """
    return sorted(scores, reverse=True)[0:3]
