#include <stdio.h>
#include <math.h>

int main(){
  int n;
  int i;
   while(scanf("%d",&n)!=EOF){

    if(n==0)break;

      int m = (int)sqrt(n);
      int s = 0;

      for(i=1;i<=m;i++){
        if(n%i==0){
          s += i ;
          s += (n/i);
        }
      }


       if(n==m*m){s-=m;}
        s -= n;

      if(s<n){printf("deficient number\n");}
      else if(s==n){printf("perfect number\n");}
      else{printf("abundant number\n");}
   }
   return 0;
}