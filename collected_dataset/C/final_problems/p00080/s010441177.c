#include <stdio.h>
#include <math.h>

int main(int argc,char* argv[]){
  
  for(;;){
    int q;
    double x;

    scanf("%d",&q);
    if(q == -1){break;}
    
    x = q/2;
    while(!(fabs(x*x*x - q) < 0.00001*q)){
      x = x - (x*x*x-q)/(3*x*x);
    }

    printf("%lf\n",x);
  }

  return 0;
  
}