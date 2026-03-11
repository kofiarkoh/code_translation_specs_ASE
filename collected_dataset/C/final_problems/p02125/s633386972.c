#include <stdio.h>

int main()
{
  unsigned int n;
  int count = 0;
  unsigned int bit = 1;
  
  scanf("%d", &n);

  for (; n / 2; n /=2) {
    if (!(n % 2)) bit = 0;
    count++;
  }

  printf("%d\n", count + bit);

  return 0;
}
  

