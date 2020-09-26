class Matrix:
    def __init__(self, matrix_string):
        self.matrix = list(map(lambda r: list(map(lambda c: int(c), r.split())), matrix_string.split('\n')))

    def row(self, index):
        return self.matrix[index-1]

    def column(self, index):
        return list(map(lambda r: r[index-1], self.matrix))
