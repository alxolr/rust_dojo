function isAnagram(word1, word2) {
  const frequency1 = new Array(26).fill(0);
  const frequency2 = new Array(26).fill(0);
  const start = "a".charCodeAt(0);

  for (const ch of word1) {
    const idx = ch.charCodeAt(0) - start;
    frequency1[idx]++;
  }

  for (const ch of word2) {
    const idx = ch.charCodeAt(0) - start;
    frequency2[idx]++;
  }

  for (let idx = 0; idx < 26; idx++) {
    if (frequency1[idx] !== frequency2[idx]) {
      return false;
    }
  }

  return true;
}

var removeAnagrams = function (words) {
  const stack = [];

  while (words.length > 0) {
    const word = words.shift();
    if (stack.length == 0) {
      stack.push(word);
    } else {
      const last = stack.pop();

      if (isAnagram(word, last)) {
        // push only the word at IMinusOne
        stack.push(last);
      } else {
        stack.push(...[last, word]);
      }
    }
  }

  return stack;
};

export default removeAnagrams;
