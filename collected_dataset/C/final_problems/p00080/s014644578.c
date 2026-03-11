#include <stdio.h>
#include <math.h>

int main(void){
    int q;
    double p, x, y, n, m;

   while(scanf("%d", &q) != -1){
       if(q == -1) break;
       p = (double)q;
       m = 0.00001*p;
       x = p/2;
       y = x*x*x-p;
       n = fabs(y);
       
       do{
           x = x - (x*x*x-p)/(3*x*x);
           y = x*x*x-p;
           n = fabs(y);
       }while(n>=m);
       printf("%.6f\n", x);
   }
   
   return 0;
   
}

