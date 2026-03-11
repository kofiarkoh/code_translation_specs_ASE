#include <stdio.h>

int main(int argc, char const *argv[]) {
  int N,count=0,countA=0,countB=0,i;
  char array[12];
  scanf("%d", &N);
  while (N > 0) {
    scanf("%s",array);
    for (i = 0; array[i+1] != '\0'; i++) {
      if (array[i] == 'A') {
        if (array[i+1] == 'B') {
          count++;
        }
      }
    }
    if (array[0] == 'B') {
      countB++;
    }
    if (array[i] == 'A') {
      countA++;
    }
    for (i = 0; array[i] != '\0'; i++) {
      array[i] = '\0';
    }
    N--;
  }
  if (countA > countB) {
    count += countB;
  } else {
    count += countA;
  }
  printf("%d\n",count);
  return 0;
}