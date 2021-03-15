DEFAULT_STUDENTS = [
    "Alice", "Bob", "Charlie", "David",
    "Eve", "Fred", "Ginny", "Harriet",
    "Ileana", "Joseph", "Kincaid", "Larry"
]

FLOWERS = [ "Violets", "Grass", "Radishes", "Clover" ]

FLOWERS_MAP = { flower[0].upper():flower for flower in FLOWERS }

class Garden:
    def __init__(self, diagram, students=None):
        students = DEFAULT_STUDENTS if not students else sorted(students)
        self.flowers = { student: [ FLOWERS_MAP[f] for r in diagram.split('\n')
                                   for f in r[i*2:i*2+2] ]
                        for i, student in enumerate(students) }

    def plants(self, student):
        return self.flowers[student]
