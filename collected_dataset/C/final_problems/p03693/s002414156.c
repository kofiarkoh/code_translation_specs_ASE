#include <stdio.h>
   int main(void){
      int r, g, b;
      int num;
      scanf("%d%d%d", &r, &g, &b);
      num = r*100 + g*10 + b;
      if(num % 4 != 0)
         printf("NO\n");
      else printf("YES\n");
}