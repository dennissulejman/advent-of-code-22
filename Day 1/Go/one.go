package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	elvesCarryingCaloriesInputFile := "../input.txt"
	readFile, err := os.Open(elvesCarryingCaloriesInputFile)

	if err != nil {
		fmt.Println(err)
	}
	fileScanner := bufio.NewScanner(readFile)

	fileScanner.Split(bufio.ScanLines)

	currentElfCaloriesCarried := 0
	highestAmountOfCaloriesCarriedByElf := 0
	for fileScanner.Scan() {

		currentLineCaloriesString := fileScanner.Text()
		if currentLineCaloriesString == "" {
			if highestAmountOfCaloriesCarriedByElf < currentElfCaloriesCarried {
				highestAmountOfCaloriesCarriedByElf = currentElfCaloriesCarried
			}
			currentElfCaloriesCarried = 0
		} else {
			currentLineCalories, _ := strconv.Atoi(currentLineCaloriesString)
			currentElfCaloriesCarried += currentLineCalories
		}
	}

	readFile.Close()

	fmt.Println(highestAmountOfCaloriesCarriedByElf)
}
