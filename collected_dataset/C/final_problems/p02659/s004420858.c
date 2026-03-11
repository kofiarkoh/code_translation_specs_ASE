/* abc_169_c */

#include<stdio.h>

int main()
{
  long long A,C,D,E;
  float B;
  scanf("%lld",&A);
  scanf("%f",&B);
  D=B*100;
  E=A*D;
  C=E/100;
  printf("%lld\n",C);
  return 0;
}


