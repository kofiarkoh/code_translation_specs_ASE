#include<stdio.h>
int main(void){
   int x, y, a, b, r, sum;
   scanf("%d %d", &x, &y);
   sum = x + y + 2;
   a = x; b = y;
   if (x < y) { b = x; a = y; }
   r = a % b;
   while ( r != 0) {
      a = b;
      b = r;
      r = a % b;
   }
   sum -= ( b + 1);
   printf("%d\n", sum);
   return 0;
}