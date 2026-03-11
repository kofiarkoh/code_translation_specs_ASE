#include <stdio.h>

int main()
{
  int kk, aa, bb;
  scanf("%d", &kk);
  scanf("%d %d", &aa, &bb);
  if(aa / kk == bb / kk && aa != bb)
  {
    puts("NG");
  }
  else
  {
    puts("OK");
  }
  return 0;
}