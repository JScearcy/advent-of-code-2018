
using System;
using System.Text.RegularExpressions;
using System.Linq;

namespace day4
{
    public static class SleepTimeRegex
    {
        private static Regex regex = new Regex(@"^\[\d{4}-\d{2}-\d{2}\s\d{2}:(\d{2})\]");

        public static int GetValue(string line)
        {
            return LineRegex.GetIntValue(SleepTimeRegex.regex, line);
        }
    }
}