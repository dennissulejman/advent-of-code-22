using AdventOfCode2;

public class Match
{
    public int OpponentScore { get; private set; } = 0;
    public int PlayerScore { get; private set; } = 0;

    public void AddNewRoundScores(IShape opponentShape, IShape playerShape)
    {
        int opponentRoundScore = 0;
        int playerRoundScore = 0;

        opponentRoundScore += opponentShape.Score;
        playerRoundScore += playerShape.Score;
        SetScores(opponentShape, playerShape, ref opponentRoundScore, ref playerRoundScore);
    }

    private void SetScores(
        IShape opponentShape,
        IShape playerShape,
        ref int opponentRoundScore,
        ref int playerRoundScore
    )
    {
        if (opponentShape.Name == playerShape.Name)
        {
            opponentRoundScore += Constants.Scores.MatchResults.Draw;
            playerRoundScore += Constants.Scores.MatchResults.Draw;
        }
        if (opponentShape.WeakerThan == playerShape.Name)
        {
            opponentRoundScore += Constants.Scores.MatchResults.Loss;
            playerRoundScore += Constants.Scores.MatchResults.Win;
        }
        if (opponentShape.StrongerThan == playerShape.Name)
        {
            opponentRoundScore += Constants.Scores.MatchResults.Win;
            playerRoundScore += Constants.Scores.MatchResults.Loss;
        }

        OpponentScore += opponentRoundScore;
        PlayerScore += playerRoundScore;
    }
}
