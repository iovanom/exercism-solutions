package strain

// type Ints []int
type Lists [][]int
type Strings []string

type Items[T any] []T

type Ints Items[int]

func keep[T any](input []T, filter func(T) bool) []T {
	var res []T
	for _, v := range input {
		if filter(v) {
			res = append(res, v)
		}
	}
	return res
}

func discard[T any](input []T, filter func(T) bool) []T {
	var res []T
	for _, v := range input {
		if !filter(v) {
			res = append(res, v)
		}
	}
	return res
}

func (i Items[T]) Keep(filter func(T) bool) Items[T] {
	return keep(i, filter)
}

func (i Ints) Discard(filter func(int) bool) Ints {
	return discard(i, filter)
}

func (l Lists) Keep(filter func([]int) bool) Lists {
	return keep(l, filter)
}

func (s Strings) Keep(filter func(string) bool) Strings {
	return keep(s, filter)
}
