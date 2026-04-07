package gross

// Units stores the Gross Store unit measurements.
func Units() map[string]int {
	m := make(map[string]int, 6)
	m["quarter_of_a_dozen"] = 3
	m["half_of_a_dozen"] = 6
	m["dozen"] = 12
	m["small_gross"] = 120
	m["gross"] = 144
	m["great_gross"] = 1728

	return m
}

// NewBill creates a new bill.
func NewBill() map[string]int {
	return make(map[string]int)
}

// AddItem adds an item to customer bill.
func AddItem(bill, units map[string]int, item, unit string) bool {
	u, ok := units[unit]
	if ok {
		bill[item] += u
		return true
	}

	return false

}

// RemoveItem removes an item from customer bill.
func RemoveItem(bill, units map[string]int, item, unit string) bool {
	u, ok := units[unit]
	if !ok {
		return false
	}

	i, ok := bill[item]
	if !ok {
		return false
	}

	diff := i - u

	if diff < 0 {
		return false
	}

	if diff == 0 {
		delete(bill, item)
	} else {
		bill[item] = diff
	}

	return true
}

// GetItem returns the quantity of an item that the customer has in his/her bill.
func GetItem(bill map[string]int, item string) (int, bool) {
	v, ok := bill[item]

	return v, ok
}
