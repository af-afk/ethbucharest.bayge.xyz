#!/usr/bin/env python3

import hashlib, math

PIECE_PAWN = 1
PIECE_KNIGHT = 2
PIECE_BISHOP = 3
PIECE_CASTLE = 4
PIECE_QUEEN = 5
PIECE_KING = 6

BOARD_SIZE = 20

ROW_SIZE = math.isqrt(BOARD_SIZE)

MAX_TRIES = 10_000

MAX_KING_MOVES = 100

def pos_to_xy(pos):
	return pos % ROW_SIZE, pos // ROW_SIZE

def xy_to_pos(x, y):
	return y * ROW_SIZE + x

def in_bounds(x, y):
	return 0 <= x < ROW_SIZE and 0 <= y < ROW_SIZE

def is_checkmate(board, king_pos):
	king_x, king_y = pos_to_xy(king_pos)
	king_nonce = board[king_pos][1]

	def is_threatened(x, y):
		if not in_bounds(x, y): return False
		test_pos = xy_to_pos(x, y)
		temp_board = board.copy()
		temp_board[test_pos] = (PIECE_KING, king_nonce)
		return is_in_check(temp_board, test_pos)

	if not is_in_check(board, king_pos):
		return False

	# Try all adjacent squares for escape
	for dx in [-1, 0, 1]:
		for dy in [-1, 0, 1]:
			if dx == 0 and dy == 0: continue
			x, y = king_x + dx, king_y + dy
			if in_bounds(x, y):
				pos = xy_to_pos(x, y)
				if pos not in board or board[pos][0] != PIECE_KING:
					if not is_threatened(x, y):
						return False  # has a safe move

	# No escape, return earliest threat
	return is_in_check(board, king_pos)

def is_in_check(board, king_pos):
	king_x, king_y = pos_to_xy(king_pos)
	king_nonce = board[king_pos][1]
	threats = []

	# Pawn (attacks up):
	for dx in [-1, 1]:
		x, y = king_x + dx, king_y - 1
		if in_bounds(x, y):
			pos = xy_to_pos(x, y)
			if pos in board and board[pos][0] == PIECE_PAWN:
				threats.append(board[pos][1])

	# Knight:
	for dx, dy in [(-2,-1), (-2,1), (-1,-2), (-1,2), (1,-2), (1,2), (2,-1), (2,1)]:
		x, y = king_x + dx, king_y + dy
		if in_bounds(x, y):
			pos = xy_to_pos(x, y)
			if pos in board and board[pos][0] == PIECE_KNIGHT:
				threats.append(board[pos][1])

	# Rook / Queen (lines):
	for dx, dy in [(-1,0), (1,0), (0,-1), (0,1)]:
		x, y = king_x, king_y
		while True:
			x += dx
			y += dy
			if not in_bounds(x, y): break
			pos = xy_to_pos(x, y)
			if pos in board:
				piece, nonce = board[pos]
				if piece in (PIECE_CASTLE, PIECE_QUEEN):
					threats.append(nonce)
				break

	# Bishop / Queen (diagonals)
	for dx, dy in [(-1,-1), (-1,1), (1,-1), (1,1)]:
		x, y = king_x, king_y
		while True:
			x += dx
			y += dy
			if not in_bounds(x, y): break
			pos = xy_to_pos(x, y)
			if pos in board:
				piece, nonce = board[pos]
				if piece in (PIECE_BISHOP, PIECE_QUEEN):
					threats.append(nonce)
				break

	# Other kings:
	for dx in [-1, 0, 1]:
		for dy in [-1, 0, 1]:
			if dx == 0 and dy == 0: continue
			x, y = king_x + dx, king_y + dy
			if in_bounds(x, y):
				pos = xy_to_pos(x, y)
				if pos in board:
					piece, nonce = board[pos]
					if piece == PIECE_KING and nonce != king_nonce:
						threats.append(nonce)

	return min(threats) if threats else False

def hash(x, y):
	d = hashlib.sha256()
	d.update(x + bytes(y))
	return d.digest()

def solve(h, target):
	b = {} # The board.
	last_king_pos = None
	for i in range(0, MAX_TRIES):
		e = int.from_bytes(hash(h, i))
		p = e & PIECE_KING
		l = (e >> 32) & BOARD_SIZE
		b[l] = [p, i]
		if p == PIECE_KING: last_king_pos = l
		if last_king_pos == None:
			continue
		# Since we've had a king placed, we need to start searching if it has
		# freedom of movement to escape to a safe location.
		piece_found = is_checkmate(b, last_king_pos)
		if piece_found:
			return (piece_found, i)
	return

if __name__ == "__main__":
	lowest, upper = solve(hash(b"test", 0), 2))
