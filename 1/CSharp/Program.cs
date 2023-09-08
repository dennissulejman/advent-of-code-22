string inputFileName = @"..\input.txt";

using StreamReader reader = new(inputFileName);

string? currentLine;
int highestAmountOfCaloriesCarriedByElf = 0;
int amountOfCaloriesCarriedByCurrentElf = 0;

while ((currentLine = reader.ReadLine()) is not null)
{
    if (string.IsNullOrWhiteSpace(currentLine))
    {
        highestAmountOfCaloriesCarriedByElf =
            amountOfCaloriesCarriedByCurrentElf > highestAmountOfCaloriesCarriedByElf
                ? amountOfCaloriesCarriedByCurrentElf
                : highestAmountOfCaloriesCarriedByElf;

        amountOfCaloriesCarriedByCurrentElf = 0;
    }
    else
    {
        amountOfCaloriesCarriedByCurrentElf += int.Parse(currentLine!);
    }
}

Console.WriteLine(highestAmountOfCaloriesCarriedByElf);