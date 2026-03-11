#include <stdio.h>
int main(void){
  long long int N,A,B;
  if(scanf("%lld%lld%lld",&N,&A,&B)<3) return 1;
  
  if((B-A)%2==0){
    printf("%lld",(B-A)/2);
  }
  else{
    if((A-1)<=(N-B)){
      printf("%lld",(A-1)+((B-A+1)/2));
    }
    else{
      printf("%lld",(N-B)+(B-A+1)/2);
    }
  }
  return 0;
}