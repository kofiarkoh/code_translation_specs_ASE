#include <stdio.h>

int main() {
  int n, s, i;
  scanf("%d %d", &n, &s);
  
  int arr[n];

  for (i = 0; i < n; i++) {
    scanf("%d", &arr[i]);
  }

  int curSum = arr[0], curSize = 0, smallestSize = n + 1;
  int left = 0, right = 0;

  while (1) {
    if (right > n - 1) {
      break;
    }

    //printf("curSum is: %d, left is: %d, right is: %d\n", curSum, left, right);
    if (curSum >= s) {
      curSize = right + 1 - left;
      smallestSize = smallestSize < curSize ? smallestSize : curSize;
      curSum -= arr[left];
      left++;
    } else {
      right++;
      curSum += arr[right];
    }
  }

  if (smallestSize == n + 1)
    smallestSize = 0;

  printf("%d\n", smallestSize);
}

