#include <stdio.h>
int main(){
   int n,flag=1;
   long long int ans=0;
   scanf("%d",&n);
   for(int i=1;i<n;i++){
       for(int j=1;flag;j++){
           if(i*j<n)ans++;
           else flag=0;
       }
     flag=1;
   }
  printf("%lld",ans);
   return 0;
}
