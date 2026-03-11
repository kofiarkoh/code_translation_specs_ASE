#include<stdio.h>
int main(void){
   int a, b, c, d;
   scanf("%d %d %d %d", &a, &b, &c, &d);
   if ( a + b == c + d && a * b == c * d ) printf("yes\n");
   else if ( a + c == b + d && a * c == b * d ) printf("yes\n");
   else if ( a + d == b + c && a * d == b * c ) printf("yes\n");
   else printf("no\n");
   return 0;
}