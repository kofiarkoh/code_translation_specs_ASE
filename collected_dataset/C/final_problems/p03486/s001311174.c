#include <stdio.h>
#include <string.h>

int main() {
  char s[101] = {0}, t[101] = {0};
  int i, j;
  int sl, tl;
  scanf("%s %s", s, t);
  sl = strlen(s);
  tl = strlen(t);

  for (i = 0; i < sl-1; i++) {
    for (j = i+1; j < sl; j++) {
      if (s[i] > s[j]) {
        char tmp;
        tmp = s[i];
        s[i] = s[j];
        s[j] = tmp;
      }
    }
  }

  for (i = 0; i < tl-1; i++) {
    for (j = i+1; j < tl; j++) {
      if (t[i] < t[j]) {
        char tmp;
        tmp = t[i];
        t[i] = t[j];
        t[j] = tmp;
      }
    }
  }

  if (strcmp(s, t) < 0) {
    puts("Yes");
  }
  else{
    puts("No");
  }
  return 0;
}
