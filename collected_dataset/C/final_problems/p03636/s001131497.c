#include <stdio.h>
#include <string.h>

char buf[55];

int main() {
  scanf("%s", buf);
  int len = strlen(buf);
  printf("%c%d%c\n", buf[0], len-2, buf[len-1]);
}
