#include<stdio.h>
int main(){
double a,b,x1,x2,y1,y2,xq,yq,x,y,c,d;
while(scanf("%lf,%lf,%lf,%lf,%lf,%lf\n",&x1,&y1,&x2,&y2,&xq,&yq)!=EOF){
a=x2-x1,b=y2-y1;
c=xq-x1,d=yq-y1;
x=(2*d*a*b+a*a*c-b*b*c)/(a*a+b*b);
y=(b*b*d-a*a*d+2*a*b*c)/(a*a+b*b);
x=x+x1,y=y+y1;
printf("%lf %lf\n",x,y);
}
return 0;
}
