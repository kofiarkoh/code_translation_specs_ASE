#include <stdio.h>

#define Num 20000

int main(void){

   int s[Num], t[Num];
   int n, q, i, j, count = 0;

   scanf("%d", &n);
   for (i=0; i < n; i++) {
      scanf(" %d", &s[i]);
   }

   scanf("%d", &q);
   for (i=0; i < q; i++) {
      scanf(" %d", &t[i]);
      for (j=0; j < n; j++) {
         if(s[j] == t[i]){
            count++;
            s[j] = 0;
            break;
         }
      }
   }

   printf("%d\n", count);

   return 0;
}

