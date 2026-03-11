#include<stdio.h>
#include<math.h>
#include<string.h>
int main(){
  
  long long int n;
  scanf("%lld",&n);
  long long int m=ceil(sqrt(n));
  long long int min;
  
  for(long long int i=1;i<m+1;i++){
    if(n%i==0){
      min=n/i;
    }
  }
  long long int a=n/min;
  
  long long int ans=a+min-2;
  
  printf("%lld\n",ans);
}
  
