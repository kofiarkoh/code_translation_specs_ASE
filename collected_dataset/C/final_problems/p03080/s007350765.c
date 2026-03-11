#include <stdio.h>

void Red_or_Blue(int N) {
  char s[N+1];
  int red, blue;
  for (scanf("%s",s), red = 0, blue = 0; red+blue < N; red++, blue++) {
    if (s[red+blue] == 'R') {
      blue -= 1;
    } else {
      red -= 1;
    }
  }
  if (red > blue) {
    printf("Yes");
  } else {
    printf("No");
  }
}

int main(void) {
  int N;
  scanf("%d",&N);
  Red_or_Blue(N);
  return 0;
}