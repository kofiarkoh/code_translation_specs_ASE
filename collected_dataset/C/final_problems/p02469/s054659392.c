#include<stdio.h>
int gcd(int a,int b){return b?gcd(b,a%b):a;}
int lcm(int a,int b){return a*(b/gcd(a,b));}
int main(){
  int n,a,b;
  scanf("%d",&n);
  for(scanf("%d",&a);--n;a=lcm(a,b))scanf("%d",&b);
  printf("%d\n",a);
  return 0;
}