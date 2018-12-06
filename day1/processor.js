const { readInput, writeResults } = require('../utils');

readInput((dataArr) => {
    const frequencyResults = findFrequencyInfo(0, new Set(), dataArr);

    writeResults(frequencyResults);
});

function findFrequencyInfo(currentFrequency, foundFrequencies, frequencyChangeList) {
    const frequencyResults = frequencyChangeList.reduce((driftAcc, currDrift) => {
        driftAcc.total += parseInt(currDrift);
        if (driftAcc.frequencies.has(driftAcc.total) && driftAcc.firstRepeatFrequency === null) {
            driftAcc.firstRepeatFrequency = driftAcc.total;
        }
        driftAcc.frequencies.add(driftAcc.total);
        return driftAcc;
    }, { total: currentFrequency, frequencies: foundFrequencies, firstRepeatFrequency: null });

    if (frequencyResults.firstRepeatFrequency !== null) {
        return frequencyResults;
    } else {
        return findFrequencyInfo(frequencyResults.total, frequencyResults.frequencies, frequencyChangeList);
    }
}