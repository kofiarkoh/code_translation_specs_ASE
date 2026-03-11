#include<stdio.h>

int main(void){
  long long n , t;
  scanf("%lld %lld",&n ,&t);

  long long a[n];
  for(int i = 0;i < n;i ++){
    scanf("%lld",&a[i]);
  }

  long long ans = 0;

  for(int i = 0;i< n-1;i ++){
    if(a[i+1] - a[i] > t){
      ans += t;
    }
    else{
      ans += a[i+1] - a[i];
    }
  }
  
  ans += t;
  printf("%lld\n",ans);

  return 0;
  
}