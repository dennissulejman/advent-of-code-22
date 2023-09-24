namespace AdventOfCode2;

public static class Constants
{
    public static class Scores
    {
        public static class MatchResults
        {
            public const int Draw = 3;
            public const int Loss = 0;
            public const int Win = 6;
        }

        public static class Shapes
        {
            public const int Paper = 2;
            public const int Rock = 1;
            public const int Scissors = 3;
        }
    }

    public static class Shapes
    {
        public const string Paper = "Paper";
        public const string Rock = "Rock";
        public const string Scissors = "Scissors";

        public static class Opponent
        {
            public const char Paper = 'B';
            public const char Rock = 'A';
            public const char Scissors = 'C';
        }

        public static class Player
        {
            public const char Paper = 'Y';
            public const char Rock = 'X';
            public const char Scissors = 'Z';
        }
    }
}
