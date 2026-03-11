#include <stdio.h>
int main() {
  int H, W;
  scanf("%d %d", &H, &W);
  while (H--) {
    char str[W];
    scanf("%s", &str);
    printf("%s\n%s\n", str, str);
  }
  return 0;
}