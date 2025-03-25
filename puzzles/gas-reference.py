#!/usr/bin/env python3

"""
Very simple (and shoddily written) Bucharest Hashing reference implementation in Python.
The implementation of the keyed hashing function is a simple sha256 using hashlib, be
mindful of this if comparing code to this implementation.
"""

import hashlib, math, unittest
from hypothesis import given, strategies as st

from copy import deepcopy

PIECE_PAWN = 1
PIECE_KNIGHT = 2
PIECE_BISHOP = 3
PIECE_CASTLE = 4
PIECE_QUEEN = 5
PIECE_KING = 6

DEFAULT_BOARD_SIZE = 0x1FF

def hash(x, y):
	"""
	Simulate a keyed hashing function using hashlib.
	"""
	d = hashlib.sha256()
	d.update(x + bytes(y))
	return d.digest()

class BucharestHashing:
	BOARD_SIZE = 0
	ROW_SIZE = 0
	MAX_TRIES = 0
	board = []
	last_king_pos = None

	def __init__(self, board_size = DEFAULT_BOARD_SIZE, max_tries = 10_000):
		self.BOARD_SIZE = board_size
		self.ROW_SIZE = math.isqrt(self.BOARD_SIZE)
		self.MAX_TRIES = max_tries
		self.board = []

	def print_board(self):
		for i in range(0, self.BOARD_SIZE):
			n = ""
			p = self.board[i][0]
			if p == PIECE_PAWN: n = "♟"
			if p == PIECE_KNIGHT: n = "♞"
			if p == PIECE_BISHOP: n = "♝"
			if p == PIECE_CASTLE: n = "♜"
			if p == PIECE_QUEEN: n = "♛"
			if p == PIECE_KING: n = "♚"
			if n == "": n = "□"
			if i == self.last_king_pos and p == PIECE_KING: n = "♔"
			print(f"| {n} ", end="")
			if (i + 1) % self.ROW_SIZE == 0: print("")
		print("")

	def pos_to_xy(self, pos):
		return pos % self.ROW_SIZE, pos // self.ROW_SIZE

	def xy_to_pos(self, x, y):
		return y * self.ROW_SIZE + x

	def in_bounds(self, x, y):
		return 0 <= x < self.ROW_SIZE and 0 <= y < self.ROW_SIZE

	def in_check_threats(self, king_pos):
		king_x, king_y = self.pos_to_xy(king_pos)
		threats = []

		# Pawn
		for dx in [-1, 1]:
			x, y = king_x + dx, king_y - 1
			if self.in_bounds(x, y):
				pos = self.xy_to_pos(x, y)
				if self.board[pos][0] == PIECE_PAWN:
					threats.append(self.board[pos][1])

		# Knight
		for dx, dy in [(-2, -1), (-2, 1), (-1,-2), (-1,2), (1,-2), (1,2), (2,-1), (2,1)]:
			x, y = king_x + dx, king_y + dy
			if not self.in_bounds(x, y): continue
			pos = self.xy_to_pos(x, y)
			if self.board[pos][0] == PIECE_KNIGHT:
				threats.append(self.board[pos][1])

		# Rook/Queen
		for dx, dy in [(-1,0), (1,0), (0,-1), (0,1)]:
			x, y = king_x, king_y
			while True:
				x += dx
				y += dy
				if not self.in_bounds(x, y): break
				pos = self.xy_to_pos(x, y)
				piece, n = self.board[pos]
				if piece == 0: continue
				if piece in (PIECE_CASTLE, PIECE_QUEEN):
					threats.append(n)
				break  # stop after first non-empty

		# Bishop/Queen
		for dx, dy in [(-1,-1), (-1,1), (1,-1), (1,1)]:
			x, y = king_x, king_y
			while True:
				x += dx
				y += dy
				if not self.in_bounds(x, y): break
				pos = self.xy_to_pos(x, y)
				piece, n = self.board[pos]
				if piece == 0: continue
				if piece in (PIECE_BISHOP, PIECE_QUEEN):
					threats.append(n)
				break  # same, stop on first hit

		# King
		for dx in [-1, 0, 1]:
			for dy in [-1, 0, 1]:
				if dx == 0 and dy == 0: continue
				x, y = king_x + dx, king_y + dy
				if not self.in_bounds(x, y): continue
				pos = self.xy_to_pos(x, y)
				piece, n = self.board[pos]
				if piece == PIECE_KING and pos != king_pos:
					threats.append(n)

		return threats

	def solve(self, starting_hash, start):
		self.last_king_pos, last_king_nonce = None, None
		self.board = [[0, 0]] * self.BOARD_SIZE
		for i in range(start, self.MAX_TRIES):
			e = int.from_bytes(hash(starting_hash, i))
			p = e % (PIECE_KING + 1)
			l = (e >> 32) % self.BOARD_SIZE
			self.board[l] = [p, i]
			if p == PIECE_KING:
				self.last_king_pos = l
				last_king_nonce = i
			if last_king_nonce == None:
				continue
			threats = self.in_check_threats(self.last_king_pos)
			if len(threats) > 0:
				threats.append(last_king_nonce)
				return min(threats), i
		return None

class TestBucharestHashing(unittest.TestCase):
	@given(starting_hash=st.binary(min_size=32, max_size=32))
	def test_prev_checks_same(self, starting_hash):
		"""
		We try to hash, then test if the proof will produce the same results.
		"""
		b = BucharestHashing()
		solution = b.solve(starting_hash, 0)
		assert solution is not None
		lowest_expected, highest_expected = solution
		c = deepcopy(b)
		lowest_test, highest_test = c.solve(starting_hash, lowest_expected)
		if lowest_expected != lowest_test:
			print(f"Lowest inconsistent, expected: {lowest_expected}, test: {lowest_test}")
			print("BEFORE")
			b.print_board()
			print("AFTER")
			c.print_board()
		if highest_expected != highest_test:
			print(f"Highest inconsistent, expected: {highest_expected}, test: {highest_test}")
			b.print_board()
			print("AFTER")
			c.print_board()
		assert lowest_expected is lowest_test, f"lowest expected ({lowest_expected}) != lowest test ({lowest_test})"
		assert highest_expected is highest_test, f"highest expected ({highest_expected}) != highest_test ({highest_test})"

if __name__ == "__main__":
	unittest.main()
