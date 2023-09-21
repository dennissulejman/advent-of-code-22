string inputFileName = @"..\input.txt";

List<int> amountOfCaloriesCarriedByElves = new();

int amountOfCaloriesCarriedByCurrentElf = 0;
foreach (string line in File.ReadLines(inputFileName))
{
    if (string.IsNullOrWhiteSpace(line))
    {
        amountOfCaloriesCarriedByElves.Add(amountOfCaloriesCarriedByCurrentElf);
        amountOfCaloriesCarriedByCurrentElf = 0;
    }
    else
    {
        amountOfCaloriesCarriedByCurrentElf += int.Parse(line);
    }
}

// The last line is a number and not whitespace so the last value needs to be added after the enumeration
amountOfCaloriesCarriedByElves.Add(amountOfCaloriesCarriedByCurrentElf);

IEnumerable<int> topThreeAmountOfCaloriesCarriedByElves = amountOfCaloriesCarriedByElves
    .OrderByDescending(amount => amount)
    .Take(3);

int highestAmountOfCaloriesCarriedByElf = topThreeAmountOfCaloriesCarriedByElves.ElementAt(0);
int sumOfTopThreeAmountOfCaloriesCarriedByElves = topThreeAmountOfCaloriesCarriedByElves.Sum(
    amount => amount
);

// Part 1
Console.WriteLine(highestAmountOfCaloriesCarriedByElf);

// Part 2
Console.WriteLine(sumOfTopThreeAmountOfCaloriesCarriedByElves);
