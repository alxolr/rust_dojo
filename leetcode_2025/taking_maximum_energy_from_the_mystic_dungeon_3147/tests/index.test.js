const { maximumEnergy } = require("../src/index");

describe("LeetCode 3147: Taking Maximum Energy from the Mystic Dungeon", () => {
  const scenarios = [
    {
      input: { energy: [5, 2, -10, -5, 1], k: 3 },
      expected: 3,
    },
    {
      input: { energy: [-2, -3, -1], k: 2 },
      expected: -1,
    },
  ];

  scenarios.forEach(({ input, expected }) => {
    test(`Input: ${JSON.stringify(input)}, Expected: ${expected}`, () => {
      expect(maximumEnergy(input.energy, input.k)).toBe(expected);
    });
  });
});
