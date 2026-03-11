//1009
#include <stdio.h>

int main()
{
   int a, b, i, n;

   while (scanf("%d %d", &a, &b) != EOF) {
       n = 1;
       for (i = 2; i <= a; i++) {
           if (a % i == 0 && b % i == 0) {
        n = i;
        }
    }
    printf("%d\n", n);
   }
   return 0;
}