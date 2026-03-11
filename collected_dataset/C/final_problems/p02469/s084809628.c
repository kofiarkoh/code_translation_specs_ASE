#include <stdio.h>

int gcd(int a, int b){
  if(b==0)
    return a;
  else
    return gcd(b,a%b);
}

int lcm(int a, int b){
  if(a < b)
    return a*b/gcd(b,a);
  else
    return a*b/gcd(a,b);
}

int main(){
  int i,n,gcd1;
  scanf("%d",&n);
  int a[n];

  for(i=0; i<n; i++)
    scanf("%d",&a[i]);
  
  gcd1 = lcm(a[0],a[1]);
  
  for(i=2; i<n; i++)
    gcd1 = lcm(gcd1,a[i]);
    
  printf("%d\n",gcd1);
  return 0;
}