using System.Collections.Generic;
using System.Linq;

namespace day4 {
    public class GuardsInfo {
        public Dictionary<int, string[]> Dict { get; set; }

        public GuardsInfo(string[] inputText) {
            this.Dict = new Dictionary<int, string[]>();
            this.initDict(inputText);
        }

        private void initDict(string[] inputText) {
            int currentKey = -1;
            foreach (string input in inputText)
            {
                string[] splitInput = input.Split("] ");
                int guardId = GuardIdRegex.GetValue(splitInput[1]);
                if (guardId != -1)
                {
                    currentKey = guardId;
                }
                else
                {
                    string[] currValue = null;
                    bool gotValue = this.Dict.TryGetValue(currentKey, out currValue);
                    if (gotValue)
                    {
                        currValue = currValue.Append(input).ToArray();
                    }
                    else
                    {
                        currValue = new string[] { input };
                    }
                    this.Dict[currentKey] = currValue;
                }
            }
        }
    }
}