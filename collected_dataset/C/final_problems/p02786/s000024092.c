  #include <stdio.h>
  #include <math.h>
  #include <stdlib.h>
  int logarithme(long long int x, long long int base)
{
  return log(x) / log(base);
}
  int main() {
    long long int H,C=1,tem=1;
    int L;
    scanf("%lld",&H);
    L=logarithme(H,2);
    for (int i = 0; i <L; i++) {
      C+=tem*2;
      tem*=2;
    }
    printf("%lld",C);
    return 0;
  }
