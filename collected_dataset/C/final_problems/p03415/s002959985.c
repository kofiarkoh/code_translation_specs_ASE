#include <stdio.h>

int main()
{
  char lf;
  char a[9];
  for(int i = 0; i < 3; i++) {
    scanf("%c%c%c%c", &a[3*i], &a[3*i+1], &a[3*i+2], &lf);
  }
  printf("%c%c%c\n", a[0], a[4], a[8]);
  return 0;
}
