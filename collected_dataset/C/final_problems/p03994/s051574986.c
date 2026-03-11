#include <stdio.h>
#include <string.h>

char next_char(char c, int n) {
  for (int i = 0; i < n; i++) {
    if (c == 'z') { c = 'a'; }
    else { c += 1; }
  }
  return c;
}

int main() {
  char WORD[100000];
  int N;

  scanf("%s", WORD);
  scanf("%d", &N);

  int len = strlen(WORD);

  int n = N;

  for (int i = 0; i < len; i++) {
    if (WORD[i] == 'a') { continue; }

    int required_n = (122 - WORD[i] + 1);

    if (n >= required_n) {
      WORD[i] = 'a';
      n -= required_n;
    }
    else {
      continue;
    }
  }

  if (n >= 0) {
    WORD[len-1] = next_char(WORD[len-1], n);
  }

  printf("%s\n", WORD);

  return 0;
}

