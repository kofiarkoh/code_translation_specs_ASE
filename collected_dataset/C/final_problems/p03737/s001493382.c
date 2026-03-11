#include <stdio.h>

void head(char s[]) {
  if (s[0] >= 97 && s[0] <= 122) {
    s[0] -= 32;
  }
  printf("%c", s[0]);
}

int main(void) {
  char s1[10], s2[10], s3[10];
  scanf("%s %s %s", s1, s2, s3);
  head(s1);
  head(s2);
  head(s3);
  printf("\n");
  return 0;
}