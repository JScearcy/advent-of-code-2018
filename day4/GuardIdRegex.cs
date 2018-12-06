
using System;
using System.Text.RegularExpressions;
using System.Linq;

namespace day4 {
    public static class GuardIdRegex {
        private static Regex regex = new Regex(@"^.*\s#(\d+)\s.*$");

        public static int GetValue(string line)
        {
            return LineRegex.GetIntValue(GuardIdRegex.regex, line, false);
        }
    }
}