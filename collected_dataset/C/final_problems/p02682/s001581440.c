#include<stdio.h>

int main(void){
   int a,b,c,k;
   int sum=0;
   scanf("%d %d %d %d",&a,&b,&c,&k);
   for(int i=0;i<a;i++){
      if(k<=0){
         break;
      }
      sum +=1;
      k -=1;
   }
   for(int i=0;i<b;i++){
      if(k<=0){
         break;
      }
      sum +=0;
      k -=1;
   }
   for(int i=0;i<c;i++){
      if(k<=0){
         break;
      }
      sum -=1;
      k -=1;
   }
   printf("%d",sum);
   return 0;
}
         
