import numpy as np
import time
class solver:
    def __init__(self, seed):
        self.startTime = time.time()
        self.seed = seed
        self.board = np.fromiter(self.seed, dtype="i4").reshape(9, 9)
        self.solve()
    def subgrid(self, n):
        if n in [0, 1, 2]:
            return 0
        if n in [3, 4, 5]:
            return 3
        if n in [6, 7, 8]:
            return 6
    def available(self, row, col):
        x, y = self.subgrid(row), self.subgrid(col)
        usednumbers = np.unique(np.concatenate((self.board[row, :], self.board[:, col], self.board[x:x+3, y:y+3].reshape(-1))))
        return np.delete(np.arange(0,10), usednumbers)
    def solve(self):
        for row in range(9):
            for col in range(9):
                if self.board[row,col] == 0:
                    for val in self.available(row, col):
                        self.board[row, col] = val
                        self.solve()
                        self.board[row, col] = 0
                    return
        print('Processing time: ', time.time()-self.startTime)
        print(self.board)

sudoku = solver('800000000003600000070090200050007000000045700000100030001000068008500010090000400') # Rixx' puzzle
