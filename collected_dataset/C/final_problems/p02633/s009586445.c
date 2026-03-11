#include <stdio.h>
 
int main(){
   int x,k,i;
 
  scanf("%d",&x);
   k=1;i=x;
   while (i>0) {
      i+=x;
      if (i>=360) i-=360;
      k++;
   }
   printf("%d\n",k);
   return 0;
}
