using AdventOfCode2;

public record Scissors : IShape
{
    public int Score => Constants.Scores.Shapes.Scissors;
    public string Name => Constants.Shapes.Scissors;
    public string StrongerThan => Constants.Shapes.Paper;
    public string WeakerThan => Constants.Shapes.Rock;
}
