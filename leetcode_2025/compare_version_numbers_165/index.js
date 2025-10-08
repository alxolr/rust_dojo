const assert = require("assert");

/**
 * @param {string} version1
 * @param {string} version2
 * @return {number}
 */
var compareVersion = function (version1, version2) {
  const version1Parts = version1.split(".").map((value) => parseInt(value));
  const version2Parts = version2.split(".").map((value) => parseInt(value));

  const maxLength = Math.max(version1Parts.length, version2Parts.length);
  //   console.log(maxLength, version1Parts, version2Parts);

  for (let idx = 0; idx < maxLength; idx++) {
    if (version1Parts[idx] != undefined && version2Parts[idx] != undefined) {
      if (version1Parts[idx] > version2Parts[idx]) {
        return 1;
      } else if (version1Parts[idx] < version2Parts[idx]) {
        return -1;
      }
    } else if (!!version1Parts[idx] === false && !!version2Parts[idx]) {
      return -1;
    } else if (!!version2Parts[idx] === false && !!version1Parts[idx]) {
      return 1;
    }
  }

  return 0;
};

function test() {
  const scenarios = [
    {
      version1: "1.2",
      version2: "1.10",
      expectedOutput: -1,
    },
    {
      version1: "1.01",
      version2: "1.001",
      expectedOutput: 0,
    },
    {
      version1: "1.0",
      version2: "1.0.0.0",
      expectedOutput: 0,
    },
    {
      version1: "2.0",
      version2: "1.0",
      expectedOutput: 1,
    },
    {
      version1: "1.0.1",
      version2: "1",
      expectedOutput: 1,
    },
    {
      version1: "2.2",
      version2: "2.2.0",
      expectedOutput: 0,
    },
    {
      version1: "2.2.3",
      version2: "2.2.3.2",
      expectedOutput: -1,
    },
  ];

  for (const [idx, scenario] of scenarios.entries()) {
    let result = compareVersion(scenario.version1, scenario.version2);
    assert.equal(result, scenario.expectedOutput);
    console.log("Scenario", idx + 1, "succesful");
  }
}

test();
