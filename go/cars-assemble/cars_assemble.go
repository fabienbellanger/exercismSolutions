package cars

// CalculateWorkingCarsPerHour calculates how many working cars are
// produced by the assembly line every hour
func CalculateWorkingCarsPerHour(productionRate int, successRate float64) float64 {
	if successRate == 0.0 {
		return 0.0
	}
	return float64(productionRate) * successRate / 100.0
}

// CalculateWorkingCarsPerMinute calculates how many working cars are
// produced by the assembly line every minute
func CalculateWorkingCarsPerMinute(productionRate int, successRate float64) int {
	if successRate == 0.0 {
		return 0.0
	}
	return int(float64(productionRate/60) * successRate / 100.0)
}

// CalculateCost works out the cost of producing the given number of cars
func CalculateCost(carsCount int) uint {
	return uint(95_000*(carsCount/10) + 10_000*(carsCount%10))
}
