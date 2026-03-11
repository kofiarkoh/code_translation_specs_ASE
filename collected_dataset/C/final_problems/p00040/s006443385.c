#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <errno.h>

char affine(char c, int a, int b) {
  return (a * (c - 'a') + b) % 26 + 'a';
}

int thisCheck(const char s[], int a, int b) {
  return 's' == affine(s[0], a, b)
    && 'i' == affine(s[-1], a, b)
    && 'h' == affine(s[-2], a, b)
    && 't' == affine(s[-3], a, b);
}

int thatCheck(const char s[], int a, int b) {
  return 't' == affine(s[0], a, b)
    && 'a' == affine(s[-1], a, b)
    && 'h' == affine(s[-2], a, b)
    && 't' == affine(s[-3], a, b);
}

int search(const char line[], int a, int b) {
  int i;
  char y;
  for (i = 3; i < strlen(line); ++i) {
    if (' ' == line[i]) continue;
    y = affine(line[i], a, b);
    if (('t' == y && thatCheck(line + i, a, b))
	|| ('s' == y && thisCheck(line + i, a, b))) {
      return 1;
    }
  }
  return 0;
}

void printAffineLine(char s[], int a, int b) {
  int i;
  for (i = 0; i < strlen(s); ++i) {
    putchar(' ' == s[i] ? s[i] : affine(s[i], a, b));
  }
  puts("");
}

int main(int argc, char *argv[]) {
  int n;
  scanf("%d\n", &n);
  while (n--) {
    char line[512];
    int a, b;
    scanf("%[^\n]\n", line);
    for (a = 0; a < 100; ++a) {
      for (b = 0; b < 100; ++b) {
	if (search(line, a, b)) {
	  //printf("a=%d, b=%d\n", a, b);
	  printAffineLine(line, a, b);
	  return 0;
	}
      }
    }
  }
  return 0;
}