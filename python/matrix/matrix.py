class Matrix:
    def __init__(self, matrix_string):
        matrix_lines = matrix_string.split('\n')
        self.matrix = list(
            map(lambda r: list(map(int, r.split())), matrix_lines))

    def row(self, index):
        return self.matrix[index-1]

    def column(self, index):
        return list(map(lambda r: r[index-1], self.matrix))
