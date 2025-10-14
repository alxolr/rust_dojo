import removeAnagrams from "../";
import assert from "assert";

describe(removeAnagrams.name, () => {
  const scenarios = [
    {
      input: ["abba", "baba", "bbaa", "cd", "cd"],
      expected: ["abba", "cd"],
    },

    {
      input: ["a", "b", "c", "d", "e"],
      expected: ["a", "b", "c", "d", "e"],
    },
  ];

  scenarios.forEach((scenario, idx) => {
    test("scenario " + (idx + 1), () => {
      assert.deepEqual(removeAnagrams(scenario.input), scenario.expected);
    });
  });
});
