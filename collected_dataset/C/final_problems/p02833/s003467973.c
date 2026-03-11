#include<stdio.h>
#include<string.h>
#include<stdlib.h>
#include<math.h>

int main(){
    long long int n,k;
    long long int ans=0;
    scanf("%lld",&n);
  
  if(n%2){
    printf("0");
    return 0;
  }
 for(int i=0;i<10000;i++){
   k=pow(5,i+1);
   if(n/k>=1){
    ans += n/(2*k);
   }
 }
  
  printf("%lld",ans);
  return 0;
}

