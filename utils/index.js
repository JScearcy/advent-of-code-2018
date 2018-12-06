const fs = require('fs');

module.exports = {
    readInput,
    writeResults
}

function readInput(callback) {
    fs.readFile('input.txt', (err, data) => {
        if (err) return console.error(err);

        const splitData = data
            .toString('utf8')
            .split('\n');
        callback(splitData);
    });
}

function writeResults(data) {
    fs.writeFile('results.json', JSON.stringify(data, null, '\t'), console.error);
}