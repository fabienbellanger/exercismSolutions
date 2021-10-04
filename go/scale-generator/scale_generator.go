package scale

const (
	mininor = iota + 1
)

func Scale(tonic, interval string) []string {
	result := make([]string, 0)
	interval, err := testInterval(interval)
	if err != nil {
		panic("invalid interval")
	}

	return result
}

func testInterval(i string) (interval string, err error) {
	return
}
