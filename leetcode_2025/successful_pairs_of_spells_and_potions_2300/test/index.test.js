const assert = require("assert");
const successfulPairs = require("../index.js");

describe(successfulPairs.name, () => {
  const scenarios = [
    {
      input: [[5, 1, 3], [1, 2, 3, 4, 5], 7],
      expected: [4, 0, 3],
    },
    {
      input: [[39,34,6,35,18,24,40], [27,37,33,34,14,7,23,12,22,37], 43],
      expected: [10,10,9,10,10,10,10],
    },
  ];

  for (const [idx, scenario] of scenarios.entries()) {
    it("scenario " + (idx + 1), () => {
      let output = successfulPairs(
        scenario.input[0],
        scenario.input[1],
        scenario.input[2]
      );

      assert.deepEqual(output, scenario.expected);
    });
  }
});
