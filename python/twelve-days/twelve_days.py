COMMON_STR = "On the {} day of Christmas my true love gave to me: {}."
NRS = (
    'first',
    'second',
    'third',
    'fourth',
    'fifth',
    'sixth',
    'seventh',
    'eighth',
    'ninth',
    'tenth',
    'eleventh',
    'twelfth'
)
GIFTS = (
    'a Partridge in a Pear Tree',
    'two Turtle Doves',
    'three French Hens',
    'four Calling Birds',
    'five Gold Rings',
    'six Geese-a-Laying',
    'seven Swans-a-Swimming',
    'eight Maids-a-Milking',
    'nine Ladies Dancing',
    'ten Lords-a-Leaping',
    'eleven Pipers Piping',
    'twelve Drummers Drumming'
)
def recite(start_verse, end_verse):
    def gifts_list(day):
        fday = GIFTS[0]
        if day == 0:
            return fday
        return ", ".join(reversed(GIFTS[1:day+1])) + ", and " + fday

    result = []
    for day in range(start_verse-1, end_verse):
        result.append(COMMON_STR.format(NRS[day], gifts_list(day)))
    return result
