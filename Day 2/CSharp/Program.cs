string inputFileName = "../input.txt";

Match match = new();
foreach (string line in File.ReadLines(inputFileName))
{
    IShape opponentShape = ShapeFactory.Create(line[0]);
    IShape playerShape = ShapeFactory.Create(line[2]);

    match.AddNewRoundScores(opponentShape, playerShape);
}

Console.WriteLine(match.PlayerScore);
