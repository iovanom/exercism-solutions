DEFAULT_STUDENTS = [
    "Alice", "Bob", "Charlie", "David",
    "Eve", "Fred", "Ginny", "Harriet",
    "Ileana", "Joseph", "Kincaid", "Larry"
]

FLOWERS = [ "Violets", "Grass", "Radishes", "Clover" ]

FLOWERS_MAP = { flower[0].upper():flower for flower in FLOWERS }

class Garden:
    def __init__(self, diagram, students=None):
        self.students = DEFAULT_STUDENTS
        if students:
            self.students = sorted(students)
        self.rows = diagram.split('\n')

    def plants(self, student):
        i = self.students.index(student) * 2
        return [ FLOWERS_MAP[f] for r in self.rows for f in r[i:i+2] ]
