package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
)

func main() {
	inputFile := "../input.txt"
	readFile, err := os.Open(inputFile)

	if err != nil {
		fmt.Println(err)
	}
	fileScanner := bufio.NewScanner(readFile)

	fileScanner.Split(bufio.ScanLines)

	var caloriesCarriedByElves []int
	currentElfCalories := 0

	for fileScanner.Scan() {

		currentLineCaloriesString := fileScanner.Text()
		if currentLineCaloriesString == "" {
			caloriesCarriedByElves = append(caloriesCarriedByElves, currentElfCalories)
			currentElfCalories = 0
		} else {
			currentLineCalories, _ := strconv.Atoi(currentLineCaloriesString)
			currentElfCalories += currentLineCalories
		}
	}

	readFile.Close()

	// This has to be done in order to append the last elf to the array
	caloriesCarriedByElves = append(caloriesCarriedByElves, currentElfCalories)

	sort.Sort(sort.Reverse(sort.IntSlice(caloriesCarriedByElves)))

	topThree := caloriesCarriedByElves[0:3]

	topThreeSum := 0

	for i := 0; i < len(topThree); i++ {
		topThreeSum += topThree[i]
	}

	fmt.Println(topThree[0])
	fmt.Println(topThreeSum)
}
