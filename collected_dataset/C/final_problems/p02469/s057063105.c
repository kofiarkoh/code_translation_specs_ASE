#include <stdio.h>

int gcd(int a, int b){
  return b ? gcd(b,a%b) : a;
}

int lcm(int a, int b){
  return a / gcd(a,b) * b;
}

int main(void){
  int i;
  int n, a[10];
  int ans;

  scanf("%d", &n);
  for(i=0;i<n;i++)
    scanf("%d", a+i);

  ans=a[0];
  for(i=1;i<n;i++)
    ans=lcm(ans, a[i]);

  printf("%d\n", ans);

  return 0;
}
