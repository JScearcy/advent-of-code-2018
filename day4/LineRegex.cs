
using System;
using System.Text.RegularExpressions;
using System.Linq;

namespace day4 {
    public static class LineRegex {
        public static int GetIntValue(Regex regex, string line, bool errorOnFail = true)
        {
            Match match = regex.Match(line);
            if (match.Success)
            {
                return Int32.Parse(match.Groups.Last().ToString());
            }
            else if (errorOnFail)
            {
                throw new FormatException("could not match string");
            }

            return -1;
        }
    }
}