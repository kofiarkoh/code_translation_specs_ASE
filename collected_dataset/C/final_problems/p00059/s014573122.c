#include<stdio.h>
int main(void){
   double xa1, xa2, xb1, xb2, ya1, ya2, yb1, yb2;
   while ( scanf("%lf %lf %lf %lf %lf %lf %lf %lf", &xa1, &ya1, &xa2, &ya2, &xb1, &yb1, &xb2, &yb2) != EOF ){

   if ( xa1 < xb1 && xb1 < xa2 ) {
      if ( ya1 < yb1 && yb1< ya2) printf("YES\n");
      else if (ya1 < yb2 && yb2 < ya2 ) printf("YES\n");
      else printf("NO\n");
   }
   else if (xa1 < xb2 && xb2 < xa2 ) {
      if ( ya1 < yb1 && yb1 < ya2 ) printf("YES\n");
      else if ( ya1 < yb2 && yb2 < ya2 ) printf("NO\n");
      else printf("NO\n");
   }
   else printf("NO\n");
   }
   return 0;
}