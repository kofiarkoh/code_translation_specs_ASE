#include <stdio.h>
#include <math.h>

#define PI acos(-1)

int main(){
  int a, b, c = 90, xx, yy;
  double x = 0, y= 0;

  while(1){
    scanf("%d,%d",&a,&b);
    if(a == 0 && b == 0) break;
    x += a * cos(c * PI / 180);
    y += a * sin(c * PI / 180);
    c -= b;
    xx = (int)x;
    yy = (int)y;
  }
  printf("%d\n%d\n",xx,yy);
  return 0;
}