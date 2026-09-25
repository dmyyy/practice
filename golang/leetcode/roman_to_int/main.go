package main

func romanToInt(s string) int {
	res := 0
	m := map[byte]int{
		'I': 1,
		'V': 5,
		'X': 10,
		'L': 50,
		'C': 100,
		'D': 500,
		'M': 1000,
	}
	for i := 0; i < len(s); i++ {
		curr := s[i]

		// on subtract parse two chars in a row
		subtracted := false
		if i+1 < len(s) {
			next := s[i+1]

			// check if we need to subtract
			if curr == 'I' && next == 'V' {
				res += 4
				subtracted = true
			}
			if curr == 'I' && next == 'X' {
				res += 9
				subtracted = true
			}
			if curr == 'X' && next == 'L' {
				res += 40
				subtracted = true
			}
			if curr == 'X' && next == 'C' {
				res += 90
				subtracted = true
			}
			if curr == 'C' && next == 'D' {
				res += 400
				subtracted = true
			}
			if curr == 'C' && next == 'M' {
				res += 900
				subtracted = true
			}

			if subtracted {
				i += 1
			}
		}

		if !subtracted {
			res += m[curr]
		}
	}

	return res
}
