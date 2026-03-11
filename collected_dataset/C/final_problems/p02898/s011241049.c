#include <stdio.h>

int main(){
   int n,k,a,i,h;
 
   scanf("%d %d",&n,&k);
   for (i=a=0;i<n;i++) {
      scanf("%d",&h);
      if (h>=k) a++;
   }
   printf("%d\n",a);
   return 0;
}
