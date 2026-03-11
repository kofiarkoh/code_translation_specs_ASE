#include <stdio.h>
#include <math.h>

int main(void){
  int x,h;
  double S;
  while(scanf("%d",&x)!=EOF){
    scanf("%d",&h);
    if(x==0&&h==0)  break;
    else{
      S=(double)x*((double)x+sqrt(4*h*h+x*x));
      printf("%.6f\n",S);
    }
  }
  return 0;
}