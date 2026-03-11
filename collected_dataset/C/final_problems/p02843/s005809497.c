#include <stdio.h>

int main(){
   long x,y,z;
 
   scanf("%ld",&x);
   y=x%100;
   z=x/100;
   if (y/5+1<=z) puts("1");
   else          puts("0");
   return 0;
}
