#include <stdio.h>

double gaiseki(double x0,double y0,double x1,double y1,double x2,double y2){
return(x0-x1)*(y0-y1)-(x0-x2)*(y0-y2);
}
int main(){
double x0,y0,x1,y1,x2,y2,x3,y3;
while (scanf("%lf,%lf,%lf,%lf,%lf,%lf,%lf,%lf",&x0,&y0,&x1,&y1,&x2,&y2,&x3,&y3)!=EOF){
if (gaiseki(x0,y0,x1,y1,x3,y3)>=0&&gaiseki(x1,y1,x2,y2,x3,y3)>=0&&gaiseki(x2,y2,x0,y0,x3,y3)>=0){
printf("YES\n");
}else{
printf("NO\n");
}
}
return 0;
}