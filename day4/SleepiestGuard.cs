using System;
using System.Collections.Generic;
using System.Linq;

namespace day4
{
    public class SleepiestGuard
    {
        private Lazy<KeyValuePair<int, KeyValuePair<int, List<int>>>> SleepiestGuardInfo { get; set; }

        private GuardsInfo GuardsInfo { get; set; }

        public SleepiestGuard(GuardsInfo guardsInfo)
        {
            this.GuardsInfo = guardsInfo;
            this.SleepiestGuardInfo = new Lazy<KeyValuePair<int, KeyValuePair<int, List<int>>>>(this.InitSleepiestGuardInfo);
        }

        public int GetSleepiestGuardId()
        {
            return this.SleepiestGuardInfo.Value.Key;
        }

        public int GetSleepiestGuardMinute()
        {
            var sleepiestGuardInfo = this.SleepiestGuardInfo.Value.Value;
            IEnumerable<int> sleepStartMinutes = sleepiestGuardInfo.Value.Where((val, index) => index % 2 == 0);
            IEnumerable<int> sleepEndMinutes = sleepiestGuardInfo.Value.Where((val, index) => index % 2 != 0);
            var allSleepMinutes = sleepStartMinutes
                .Zip(sleepEndMinutes, (a, b) => Tuple.Create(a, b))
                .SelectMany(tuple =>
                    Enumerable.Range(tuple.Item1, tuple.Item2 - tuple.Item1)
                ).GroupBy(x => x, (minute, groupMinutes) => new { Count = groupMinutes.Count(), Minute = minute }).OrderByDescending(x => x.Count).ToList();
            return allSleepMinutes.First().Minute;
        }

        private KeyValuePair<int, KeyValuePair<int, List<int>>> InitSleepiestGuardInfo()
        {
            return this.GuardsInfo.Dict
                .Select(pair =>
                {
                    int sleepStart = 0;
                    List<int> sleepAmounts = new List<int>();
                    List<int> sleepTimes = new List<int>();
                    foreach (string sleepString in pair.Value)
                    {
                        int sleepNumber = SleepTimeRegex.GetValue(sleepString);
                        if (sleepString.Contains("falls asleep"))
                        {
                            sleepStart = sleepNumber;
                        }
                        else
                        {
                            sleepAmounts.Add(sleepNumber - sleepStart);
                        }
                        sleepTimes.Add(sleepNumber);
                    }
                    int sleepTotal = sleepAmounts.Sum();

                    return new KeyValuePair<int, KeyValuePair<int, List<int>>>(pair.Key, new KeyValuePair<int, List<int>>(sleepTotal, sleepTimes));
                })
                .OrderByDescending(pair => pair.Value.Key)
                .ToDictionary(pair => pair.Key, pair => pair.Value)
                .First();
        }
    }
}