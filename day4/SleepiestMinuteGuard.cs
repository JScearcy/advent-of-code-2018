using System;
using System.Collections.Generic;
using System.Linq;

namespace day4
{
    public class SleepiestMinuteGuard
    {
        private Lazy<KeyValuePair<int, int>> SleepiestGuardInfo { get; set; }

        private GuardsInfo GuardsInfo { get; set; }

        public SleepiestMinuteGuard(GuardsInfo guardsInfo)
        {
            this.GuardsInfo = guardsInfo;
            this.SleepiestGuardInfo = new Lazy<KeyValuePair<int, int>>(this.InitSleepiestGuardInfo);
        }

        public int GetSleepiestMinuteGuardId()
        {
            return this.SleepiestGuardInfo.Value.Key;
        }

        public int GetSleepiestMinuteGuard()
        {
            var sleepiestGuardInfo = this.SleepiestGuardInfo.Value;
            return sleepiestGuardInfo.Value;
        }

        private KeyValuePair<int, int> InitSleepiestGuardInfo()
        {
            return this.GuardsInfo.Dict
                .Select(pair =>
                {
                    List<int> sleepTimes = new List<int>();
                    foreach (string sleepString in pair.Value)
                    {
                        sleepTimes.Add(SleepTimeRegex.GetValue(sleepString));
                    }
                    Tuple<int, int> sleepiestMinute = this.SleepiestMinute(sleepTimes);

                    return new KeyValuePair<int, Tuple<int,int>>(pair.Key, sleepiestMinute);
                })
                .OrderByDescending(pair => pair.Value.Item1)
                .ToDictionary(pair => pair.Key, pair => pair.Value.Item2)
                .First();
        }

        private Tuple<int, int> SleepiestMinute(List<int> sleepMinutes) {
            IEnumerable<int> sleepStartMinutes = sleepMinutes.Where((val, index) => index % 2 == 0);
            IEnumerable<int> sleepEndMinutes = sleepMinutes.Where((val, index) => index % 2 != 0);
            var allSleepMinutes = sleepStartMinutes
                .Zip(sleepEndMinutes, (a, b) => Tuple.Create(a, b))
                .SelectMany(tuple =>
                    Enumerable.Range(tuple.Item1, tuple.Item2 - tuple.Item1)
                )
                .GroupBy(x => x, (minute, groupMinutes) => new { Count = groupMinutes.Count(), Minute = minute })
                .OrderByDescending(x => x.Count)
                .ToList()
                .First();

            return Tuple.Create(allSleepMinutes.Count, allSleepMinutes.Minute);

        }
    }
}