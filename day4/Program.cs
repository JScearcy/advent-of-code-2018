using System;
using System.Collections.Generic;
using System.Linq;
using System.Text.RegularExpressions;

namespace day4
{
    class Program
    {
        static void Main(string[] args)
        {
            string inputText = System.IO.File.ReadAllText(@"input.txt");
            string[] orderedInputText = inputText.Split('\n').OrderBy(str => str.Split(' ', 1)[0]).ToArray();
            GuardsInfo guardsInfo = new GuardsInfo(orderedInputText);
            SleepiestGuard mostSleepy = new SleepiestGuard(guardsInfo);
            SleepiestMinuteGuard mostSleepyMinuteGuard = new SleepiestMinuteGuard(guardsInfo);
            int mostSleepyKey = mostSleepy.GetSleepiestGuardId();
            int mostSleepyMinute = mostSleepy.GetSleepiestGuardMinute();
            int mostSleepyMinuteGuardKey = mostSleepyMinuteGuard.GetSleepiestMinuteGuardId();
            int mostSleepyMinuteGuardMinute = mostSleepyMinuteGuard.GetSleepiestMinuteGuard();

            string results = Program.FormatResults(mostSleepyKey, mostSleepyMinute, mostSleepyMinuteGuardKey, mostSleepyMinuteGuardMinute);
            System.IO.File.WriteAllText("results.json", results);
        }

        private static string FormatResults(int mostSleepyKey, int mostSleepyMinute, int mostSleepyMinuteGuardKey, int mostSleepyMinuteGuardMinute) {
            return $"{{\"mostSleepy\": {{ \"id\": {mostSleepyKey}, \"minute\": {mostSleepyMinute}, \"answer\": {mostSleepyKey * mostSleepyMinute} }}, \"mostSleepyMinute\": {{ \"id\": {mostSleepyMinuteGuardKey}, \"minute\": {mostSleepyMinuteGuardMinute}, \"answer\": {mostSleepyMinuteGuardKey * mostSleepyMinuteGuardMinute} }} }}";
        }
    }
}
