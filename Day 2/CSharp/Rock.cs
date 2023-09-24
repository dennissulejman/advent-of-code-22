using AdventOfCode2;

public record Rock : IShape
{
    public int Score => Constants.Scores.Shapes.Rock;
    public string Name => Constants.Shapes.Rock;
    public string StrongerThan => Constants.Shapes.Scissors;
    public string WeakerThan => Constants.Shapes.Paper;
}
