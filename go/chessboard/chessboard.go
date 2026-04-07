package chessboard

type File []bool

type Chessboard map[string]File

// CountInFile returns how many squares are occupied in the chessboard,
// within the given file.
func CountInFile(cb Chessboard, file string) int {
	n := 0

	f := cb[file]
	for _, v := range f {
		if v {
			n += 1
		}
	}

	return n
}

// CountInRank returns how many squares are occupied in the chessboard,
// within the given rank.
func CountInRank(cb Chessboard, rank int) int {
	if rank > 8 || rank < 1 {
		return 0
	}

	n := 0
	for _, f := range cb {
		v := f[rank-1]
		if v {
			n += 1
		}
	}

	return n
}

// CountAll should count how many squares are present in the chessboard.
func CountAll(cb Chessboard) int {
	n := 0

	for _, f := range cb {
		for range f {
			n += 1
		}
	}

	return n

}

// CountOccupied returns how many squares are occupied in the chessboard.
func CountOccupied(cb Chessboard) int {
	n := 0

	for _, v := range cb {
		for _, f := range v {
			if f {
				n += 1
			}
		}
	}

	return n
}
