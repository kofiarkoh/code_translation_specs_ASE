#include <stdio.h>

int main(){
  long long i,n,a,b,c,d;

  scanf("%lld %lld %lld %lld %lld",&n,&a,&b,&c,&d);
  for(i=0;i<n;i++){
    if(c*(n-1-i)-d*i<=b-a&&b-a<=-c*i+d*(n-1-i)){
      printf("YES");
      return 0;
    }
  }
  printf("NO");
  return 0;
}
