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

int highestAmountOfCaloriesCarriedByElf = amountOfCaloriesCarriedByElves
    .OrderByDescending(amount => amount)
    .ElementAt(0);

Console.WriteLine(highestAmountOfCaloriesCarriedByElf);