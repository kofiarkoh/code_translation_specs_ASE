#include <stdio.h>

int main(void) {
   int x,y,i,temp=0;
   scanf("%d %d",&x,&y);
   for(i=0;i<=x*2;i=i+2){
       if((float)y/(2*x)*i==(float)((int)((float)y/(2*x)*i))){temp++;}
   }
   printf("%d\n",x+y+2-temp);
   return 0;
}

