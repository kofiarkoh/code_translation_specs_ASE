#include<stdio.h>
int main(void){
   int a[500], count, i, j, n, key;
   scanf("%d", &n);
   while ( n ) {
    for ( i = 0; i < 2 * n; i++ ) {
        a[i] = 0;
    }
       key = 0;
       for ( i = 0; i < n; i++ ) {
           scanf("%d", &a[i]);
       }
       while ( 1 ) {
           count = 0;
           for ( i = 0; i < n; i++ ) {
               for ( j = 0; j < n; j++ ) {
                   if ( a[i] == a[j] ) a[i + n]++;
                   /*printf("a\n");*/
               }
               if ( a[i + n] == a[i] ) count++;
           }
           if ( count == n ) break;
           for ( i = 0; i < n; i++ ) {
               a[i] = a[i + n];
               a[i + n] = 0;
           }
           /*printf("%d", a[0]);
           for( i = 1; i < n; i++ ) {
               printf(" %d", a[i]);
           }
           printf("\n");*/
   
           key++;
        }
        printf("%d\n", key);
        printf("%d", a[0]);
        for( i = 1; i < n; i++ ) {
            printf(" %d", a[i]);
        }
        printf("\n");
        scanf("%d", &n);
    }
   return 0;
}