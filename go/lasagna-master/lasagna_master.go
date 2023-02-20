package lasagna

func PreparationTime(layers []string, avgTime int) int {
	if avgTime <= 0 {
		avgTime = 2
	}

	return len(layers) * avgTime
}

func Quantities(layers []string) (noodles int, sauce float64) {
	for _, l := range layers {
		if l == "noodles" {
			noodles += 50
		} else if l == "sauce" {
			sauce += 0.2
		}
	}

	return
}

func AddSecretIngredient(friendsList, myList []string) {
	myList[0] = friendsList[0]
	myList[len(myList)-1] = friendsList[len(friendsList)-1]
}

func ScaleRecipe(quantities []float64, portions int) []float64 {
	scaledQuantities := make([]float64, len(quantities))

	for i, v := range quantities {
		scaledQuantities[i] = v * float64(portions) / 2.0
	}

	return scaledQuantities
}