using AdventOfCode2;

public record Paper : IShape
{
    public int Score => Constants.Scores.Shapes.Paper;
    public string Name => Constants.Shapes.Paper;
    public string StrongerThan => Constants.Shapes.Rock;
    public string WeakerThan => Constants.Shapes.Scissors;
}
