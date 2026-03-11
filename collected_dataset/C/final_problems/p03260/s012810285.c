#include <stdio.h>
int main() {
  int A, B;
  scanf("%d %d", &A, &B);
  if ((A & B) & 0x1) puts("Yes");
  else puts("No");
  return 0;
}