#include <stdio.h>
#include <string.h>

int main(void) {

  char s[100001];
  scanf("%s", s);
  int length = strlen(s);
  int a = 0, b = 0, c = 0;
  for (int i = 0; i < length; i++) {
    if (s[i] == 'a') {
      a++;
    } else if (s[i] == 'b') {
      b++;
    } else {
      c++;
    }
  }
  int min = a;
  if (b < min) {
    min = b;
  }
  if (c < min) {
    min = c;
  }
  if (a-min < 2 && b-min < 2 && c-min < 2) {
    printf("YES\n");
  } else {
    printf("NO\n");
  }

  return 0;
}