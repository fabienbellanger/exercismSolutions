package listops

// IntList is an abstraction of a list of integers which we can define methods on
type IntList []int

func (s IntList) Foldl(fn func(int, int) int, initial int) int {
	r := initial

	for _, v := range s {
		r = fn(r, v)
	}

	return r
}

func (s IntList) Foldr(fn func(int, int) int, initial int) int {
	r := initial

	for i := len(s) - 1; i >= 0; i-- {
		r = fn(s[i], r)
	}

	return r
}

func (s IntList) Filter(fn func(int) bool) IntList {
	list := make(IntList, 0, len(s))

	for _, v := range s {
		if fn(v) {
			list = append(list, v)
		}
	}

	return list
}

func (s IntList) Length() int {
	return len(s)
}

func (s IntList) Map(fn func(int) int) IntList {
	r := make(IntList, 0, len(s))

	for _, v := range s {
		r = append(r, fn(v))
	}

	return r
}

func (s IntList) Reverse() IntList {
	r := make(IntList, 0, len(s))

	for i := len(s) - 1; i >= 0; i-- {
		r = append(r, s[i])
	}

	return r
}

func (s IntList) Append(lst IntList) IntList {
	r := make(IntList, 0, len(s)+len(lst))

	r = append(r, s...)
	r = append(r, lst...)

	return r
}

func (s IntList) Concat(lists []IntList) IntList {
	r := make(IntList, 0)

	r = r.Append(s)

	for _, list := range lists {
		r = r.Append(list)
	}

	return r
}
