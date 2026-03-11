#include<stdio.h>

double a(double A,double a,double B,double b,double x,double y)
{
    if(A!=B){
        return ((a-b)/(A-B))*(x-A)+a-y;
    } else {
        return x-A;
    }
}

int main(void)
{
    double x1,y1,x2,y2,x3,y3,xp,yp,xs,ys;
    int i,j,n;

    for(scanf("%d",&n);n>0;n--){

        scanf("%lf%lf%lf%lf%lf%lf%lf%lf%lf%lf",&x1,&y1,&x2,&y2,&x3,&y3,&xp,&yp,&xs,&ys);

        if((a(x1,y1,x2,y2,xp,yp)*a(x1,y1,x2,y2,x3,y3)<0)||(a(x3,y3,x2,y2,xp,yp)*a(x3,y3,x2,y2,x1,y1)<0)||(a(x1,y1,x3,y3,xp,yp)*a(x1,y1,x3,y3,x2,y2)<0)){
            if((a(x1,y1,x2,y2,xs,ys)*a(x1,y1,x2,y2,x3,y3)<0)||(a(x3,y3,x2,y2,xs,ys)*a(x3,y3,x2,y2,x1,y1)<0)||(a(x1,y1,x3,y3,xs,ys)*a(x1,y1,x3,y3,x2,y2)<0)){
                puts("NG");
            } else {
                puts("OK");
            }
        } else {
            if((a(x1,y1,x2,y2,xs,ys)*a(x1,y1,x2,y2,x3,y3)<0)||(a(x3,y3,x2,y2,xs,ys)*a(x3,y3,x2,y2,x1,y1)<0)||(a(x1,y1,x3,y3,xs,ys)*a(x1,y1,x3,y3,x2,y2)<0)){
                puts("OK");
            } else {
                puts("NG");
            }
        }
    }
    return 0;
}