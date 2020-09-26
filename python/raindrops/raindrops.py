configs = ((3, "Pling"), (5, "Plang"), (7, "Plong"))

def convert(number):
    result = "".join([raindrop for factor, raindrop in configs if number % factor == 0])
    return result or str(number)

