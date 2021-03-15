from collections import namedtuple


Student = namedtuple('Student', ['grade', 'name'])

def _get_student_name(student: Student) -> str:
    return student.name

class School:
    def __init__(self):
        self.students = []

    def add_student(self, name: str, grade: int):
        self.students.append(Student(name=name, grade=grade))
        self.students.sort(key=tuple)

    def roster(self):
        return list(map(_get_student_name, self.students))

    def grade(self, grade_number: int):
        def _compare_grade(student: Student) -> bool:
            return student.grade == grade_number

        return list(map(_get_student_name, filter(_compare_grade, self.students)))
