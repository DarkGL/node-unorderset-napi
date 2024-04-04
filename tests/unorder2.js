const { UnorderedSet } = require("../index");

const set = new UnorderedSet();

const iteration = 10_000_000;

for (let i = 0; i < iteration; i++) {
    set.insert(BigInt(i));
}

for (let i = 0; i < iteration; i++) {
    set.has(BigInt(i));
}