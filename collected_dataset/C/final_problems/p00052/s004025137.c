#include <stdio.h>

int main(){
  long a,b,i,n;

  while(scanf("%ld",&n),n!=0){
    a=0;
    for(i=5;i<=n;i++){
      b=i;
      while(b>0){
        if(b%5!=0) break;
        a++;
        b=b/5;
      }
    }
    printf("%ld\n",a);
  }
  return 0;
}