var successfulPairs = function (spells, potions, success) {
  potions.sort((a, b) => a - b);
  const answer = [];

  for (const spell of spells) {
    function findMinIdx(left, right) {
      if (left < right) {
        const mid = Math.floor((left + right) / 2);

        if (potions[right] * spell < success) {
          return -1;
        }

        if (potions[mid] * spell >= success) {
          return findMinIdx(left, mid);
        } else {
          return findMinIdx(mid + 1, right);
        }
      }

      if (left == right) {
        if (potions[left] * spell >= success) {
          return left;
        } else {
          return -1;
        }
      }

      return -1;
    }

    const idx = findMinIdx(0, potions.length - 1);

    if (idx == -1) {
      answer.push(0);
    } else {
      answer.push(potions.length - idx);
    }
  }

  return answer;
};

module.exports = successfulPairs;
