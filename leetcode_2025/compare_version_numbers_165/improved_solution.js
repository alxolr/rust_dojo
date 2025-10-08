const assert = require("assert");

/**
 * Compares two version strings and returns:
 * -1 if version1 < version2
 *  0 if version1 == version2  
 *  1 if version1 > version2
 * 
 * @param {string} version1
 * @param {string} version2
 * @return {number}
 */
function compareVersion(version1, version2) {
  const parts1 = version1.split('.');
  const parts2 = version2.split('.');
  
  const maxLength = Math.max(parts1.length, parts2.length);
  
  for (let i = 0; i < maxLength; i++) {
    // Parse each part as needed, defaulting to 0 for missing parts
    const num1 = parseInt(parts1[i] || '0', 10);
    const num2 = parseInt(parts2[i] || '0', 10);
    
    if (num1 < num2) return -1;
    if (num1 > num2) return 1;
    // Continue if equal
  }
  
  return 0;
}

/**
 * Alternative solution using padEnd for cleaner logic
 */
function compareVersionAlternative(version1, version2) {
  const parts1 = version1.split('.');
  const parts2 = version2.split('.');
  
  const maxLength = Math.max(parts1.length, parts2.length);
  
  // Pad shorter array with '0' strings
  const paddedParts1 = parts1.concat(Array(maxLength - parts1.length).fill('0'));
  const paddedParts2 = parts2.concat(Array(maxLength - parts2.length).fill('0'));
  
  for (let i = 0; i < maxLength; i++) {
    const num1 = parseInt(paddedParts1[i], 10);
    const num2 = parseInt(paddedParts2[i], 10);
    
    if (num1 < num2) return -1;
    if (num1 > num2) return 1;
  }
  
  return 0;
}

/**
 * Most elegant solution using array access with fallback
 */
function compareVersionOptimal(version1, version2) {
  const v1 = version1.split('.');
  const v2 = version2.split('.');
  const maxLen = Math.max(v1.length, v2.length);
  
  for (let i = 0; i < maxLen; i++) {
    const a = +(v1[i] || 0);  // Unary plus for quick conversion
    const b = +(v2[i] || 0);
    
    if (a !== b) return a > b ? 1 : -1;
  }
  
  return 0;
}

// Test function to verify all solutions work correctly
function runTests() {
  const testCases = [
    { version1: "1.2", version2: "1.10", expected: -1 },
    { version1: "1.01", version2: "1.001", expected: 0 },
    { version1: "1.0", version2: "1.0.0.0", expected: 0 },
    { version1: "2.0", version2: "1.0", expected: 1 },
    { version1: "1.0.1", version2: "1", expected: 1 },
    { version1: "2.2", version2: "2.2.0", expected: 0 },
    { version1: "2.2.3", version2: "2.2.3.2", expected: -1 },
    { version1: "1.0.0", version2: "1", expected: 0 },
    { version1: "0.1", version2: "1.1", expected: -1 }
  ];

  const solutions = [
    { name: "Main Solution", fn: compareVersion },
    { name: "Alternative Solution", fn: compareVersionAlternative },
    { name: "Optimal Solution", fn: compareVersionOptimal }
  ];

  solutions.forEach(solution => {
    console.log(`\n=== Testing ${solution.name} ===`);
    testCases.forEach((testCase, index) => {
      const result = solution.fn(testCase.version1, testCase.version2);
      try {
        assert.strictEqual(result, testCase.expected);
        console.log(`✅ Test ${index + 1}: ${testCase.version1} vs ${testCase.version2} = ${result}`);
      } catch (error) {
        console.log(`❌ Test ${index + 1}: Expected ${testCase.expected}, got ${result}`);
      }
    });
  });
}

// Performance comparison
function performanceTest() {
  const testPairs = Array(10000).fill(0).map(() => [
    `${Math.floor(Math.random() * 10)}.${Math.floor(Math.random() * 10)}.${Math.floor(Math.random() * 10)}`,
    `${Math.floor(Math.random() * 10)}.${Math.floor(Math.random() * 10)}.${Math.floor(Math.random() * 10)}`
  ]);

  console.log('\n=== Performance Test ===');
  
  const solutions = [
    { name: "Main Solution", fn: compareVersion },
    { name: "Optimal Solution", fn: compareVersionOptimal }
  ];

  solutions.forEach(solution => {
    const start = performance.now();
    testPairs.forEach(([v1, v2]) => solution.fn(v1, v2));
    const end = performance.now();
    console.log(`${solution.name}: ${(end - start).toFixed(2)}ms`);
  });
}

if (require.main === module) {
  runTests();
  performanceTest();
}

module.exports = { compareVersion, compareVersionAlternative, compareVersionOptimal };