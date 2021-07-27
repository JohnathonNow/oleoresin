const bent = require('bent')

function main(params) {
    msg = "Hello, " + params.name + " from " + params.place;
    return { greeting:  msg };
}

exports.main = main;
