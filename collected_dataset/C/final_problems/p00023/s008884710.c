#include <stdio.h>
#include <math.h>
int main(){
  int i,n;
  double xa,ya,xb,yb,ra,rb;

  scanf("%d",&n);
  for (i=0;i<n;i++){
    scanf("%lf %lf %lf %lf %lf %lf",&xa,&ya,&ra,&xb,&yb,&rb);
    double d=sqrt(pow((xa-xb),2)+pow((ya-yb),2));
    if (d<ra-rb)
      printf("2\n");
    else if (d<rb-ra)
      printf("-2\n");
    else if (d<ra+rb)
      printf("1\n");
    else if (d>ra+rb)
      printf("0\n");
  }
  return 0;
}