#include<stdio.h>
#include<string.h>
#include<stdlib.h>
int main(){
  long long int i,a,b,v,n,k;
  scanf("%lld%lld",&n,&k);
  a=n/k;
  if((n%k)*2>=k)
    b=a+1;
  else
    b=a;
  v=0;v+=a*a*a;
  if(k%2==0)
    v+=b*b*b;
  printf("%lld",v);
}