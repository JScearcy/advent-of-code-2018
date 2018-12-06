const { readInput, writeResults } = require('../utils');

readInput((data) => {
    const sigLettersFinal = data.reduce((sigLetters, id, index) => {
        const idSigLetters = getIdSigLetters(id);
        if (sigLetters.matchesOffOne.length === 0) {
            const matchesOffOne = checkMatchesOffOne(index, data);
            if (matchesOffOne !== null) {
                sigLetters.matchesOffOne = matchesOffOne;
            }
        }

        sigLetters.two += idSigLetters.two;
        sigLetters.three += idSigLetters.three;

        return sigLetters;
    }, { two: 0, three: 0, matchesOffOne: '' });

    writeResults({ checkSum: sigLettersFinal.two * sigLettersFinal.three, matchesOffOne: sigLettersFinal.matchesOffOne });
});

function checkMatchesOffOne(index, ids) {
    let matchesOffOneLetters = null;
    const baseId = ids[index].split('');
    for (let i = index; i < ids.length; i++) {
        const currentId = ids[i];
        const { commonLetters, noMatchResults } = baseId.reduce((noMatches, letter, idx) => {
            if (letter !== currentId[idx]) {
                noMatches.noMatchResults += 1;
            } else {
                noMatches.commonLetters = `${noMatches.commonLetters}${currentId[idx]}`;
            }
            return noMatches;
        }, { commonLetters: '', noMatchResults: 0 });

        if (noMatchResults === 1) {
            matchesOffOneLetters = commonLetters;
        }
    }

    return matchesOffOneLetters;
}

function getIdSigLetters(id) {
    const letterHash = id
        .split('')
        .reduce((letterCount, letter) => {
            if (letterCount[letter]) {
                letterCount[letter] += 1;
            } else {
                letterCount[letter] = 1;
            }

            return letterCount;
        }, {});

    const sigLettersFinal = Object.keys(letterHash).reduce((sigLetters, key) => {
        if (letterHash[key] === 2) {
            sigLetters.two = 1;
        } else if (letterHash[key] === 3) {
            sigLetters.three = 1;
        }

        return sigLetters;
    }, { two: 0, three: 0 });

    return sigLettersFinal
}