package main

func myPow(x float64, n int) float64 {
	// works fine but TLE on large n

	/*
		 if n == 0 {
			return 1
		} else if n > 0 {
			res := x
			for range n - 1 {
				res = res * x
			}
			return res
		} else {
			res := x
			for range -n - 1 {
				res = res * x
			}
			return 1 / res
		}
	*/

	// slightly smarter solution

	if n < 0 {
		x = 1 / x
		n = -n
	}

	res := 1.0
	for n > 0 {
		if n%2 == 1 {
			res *= x
		}
		x *= x
		n /= 2
	}

	return res
}
