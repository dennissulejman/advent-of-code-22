package main

import (
	"bufio"
	"fmt"
	"os"
)

const paper = "paper"
const rock = "rock"
const scissors = "scissors"

const p_score int = 2
const r_score int = 1
const s_score int = 3

const draw int = 3
const loss int = 0
const win int = 6

func main() {
	input_file := "../input.txt"
	read_file, err := os.Open(input_file)

	if err != nil {
		fmt.Println(err)
	}
	file_scanner := bufio.NewScanner(read_file)

	file_scanner.Split(bufio.ScanLines)

	o_score := 0
	p_score := 0

	for file_scanner.Scan() {

		current_line := file_scanner.Text()

		o_shape := initShape(string(current_line[0]))
		p_shape := initShape(string(current_line[2]))

		o_score += o_shape.Score
		p_score += p_shape.Score

		if o_shape.Name == p_shape.Name {
			o_score += draw
			p_score += draw
		}
		if o_shape.Name == p_shape.WeakerThan {
			o_score += win
			p_score += loss
		}
		if o_shape.Name == p_shape.StrongerThan {
			o_score += loss
			p_score += win
		}
	}

	fmt.Println(p_score)
}

type Shape struct {
	Score        int
	Name         string
	StrongerThan string
	WeakerThan   string
}

func initShape(key string) *Shape {
	switch key {
	case "A", "X":
		return &Shape{
			Score:        r_score,
			Name:         rock,
			StrongerThan: scissors,
			WeakerThan:   paper,
		}
	case "B", "Y":
		return &Shape{
			Score:        p_score,
			Name:         paper,
			StrongerThan: rock,
			WeakerThan:   scissors,
		}
	case "C", "Z":
		return &Shape{
			Score:        s_score,
			Name:         scissors,
			StrongerThan: paper,
			WeakerThan:   rock,
		}

	default:
		return nil
	}
}
