#include<stdio.h>
#include<math.h>
int main(void){
   long h[10], w[10], t[10], k[10]; 
   int count = 0, i, j;
   for ( i = 1; i <= 6; i++ ) {
      scanf("%ld %ld", &h[i], &w[i]);
      t[i] = h[i] + w[i];
      k[i] = h[i] * w[i];
   }
   for ( i = 1; i <= 6; i++ ) {
      for ( j = i + 1; j <= 6; j++ ) {
/*printf("i:%d t:%d k:%d j:%d t:%d k:%d count:%d\n", i, t[i], k[i], j, t[j], k[j], count);*/
         if ( t[j] != 0 && t[i] == t[j] && k[i] == k[j]) { count++; t[j] = k[j] = 0; break;}
      }
   }
   if ( count >= 3 ) printf("yes\n");
   else printf("no\n");
   return 0;
}