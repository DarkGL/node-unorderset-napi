const map = new Map();

const iteration = 10_000_000;

for (let i = 0; i < iteration; i++) {
    map.set(i, true);
}

for (let i = 0; i < iteration; i++) {
    map.has(i);
}
