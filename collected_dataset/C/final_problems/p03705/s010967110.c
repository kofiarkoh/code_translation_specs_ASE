#include<stdio.h>

int main(void){
  int N,A,B;
  int i;
  long long max,min,ans;
  scanf("%d%d%d",&N,&A,&B);
  if(A > B){
    ans = 0;
  }else if(N == 1 && A != B){
    ans = 0;
  }else{
    min = A*(N-1)+B;
    max = B*(N-1)+A;
    ans = max-min+1;
  }
  printf("%lld\n",ans);
  return 0;
}
