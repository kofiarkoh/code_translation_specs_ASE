#include<stdio.h>
int main(void){
   int n, N[5000], sum, max, i, j;
   scanf("%d", &n);
   while ( n ) {
       max = -100000;
       for ( i = 0; i < n; i++ ) {
           scanf("%d", &N[i]);
       }
       for ( i = 0; i < n; i++ ) {
           sum = 0;
           for ( j = i; j < n; j++ ) {
               sum += N[j];
               if ( sum > max ) max = sum;
               /*printf("sum:%d\n", sum);*/
           }
       }
       printf("%d\n", max);
       scanf("%d", &n);
   }
   return 0;
}