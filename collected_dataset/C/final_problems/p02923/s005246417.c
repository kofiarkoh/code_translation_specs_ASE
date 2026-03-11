#include<stdio.h>

int main(){
  long long n,h[100001],i,max=0,num=0;
  h[0]=0;
  scanf("%lld",&n);
  for(i=1;i<=n;i++){
    scanf("%lld",&h[i]);
    if(h[i]<=h[i-1]) {
      num++;
    }
    else{
      num=0;
    }
    if(num>max)max=num;
  }

  printf("%lld",max);
  return 0;
}
