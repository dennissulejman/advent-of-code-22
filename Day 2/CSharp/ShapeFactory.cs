using AdventOfCode2;

public static class ShapeFactory
{
    public static IShape Create(char shape)
    {
        return shape switch
        {
            Constants.Shapes.Opponent.Paper or Constants.Shapes.Player.Paper => new Paper(),
            Constants.Shapes.Opponent.Rock or Constants.Shapes.Player.Rock => new Rock(),
            Constants.Shapes.Opponent.Scissors
            or Constants.Shapes.Player.Scissors
                => new Scissors(),
            _ => throw new ArgumentException($"Cannot cast shape \"{shape}\" into object")
        };
    }
}
