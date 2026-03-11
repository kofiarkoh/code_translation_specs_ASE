#include<stdio.h>
#include<math.h>
struct circle
{
    double x;
    double y;
    double radius;
}circle;

int fun(double a,double b,double c,double d, double e,double f);

int main()
{
    int n;


    double x1,y1,x2,y2,x3,y3;
    double a,b,c,d,e,f;
    int i;
    scanf("%d",&n);
    for(i=0;i<n;i++)
    {

        scanf("%lf %lf %lf %lf %lf %lf",&x1,&y1,&x2,&y2,&x3,&y3);
            a = 2*(x3-x1);
            b = 2*(y3-y1);
            c = y3*y3+x3*x3-y1*y1-x1*x1;
            d = 2*(x2-x1);
            e = 2*(y2-y1);
            f = y2*y2+x2*x2-y1*y1-x1*x1;
            if (a != 0)
            {
                fun(a,b,c,d,e,f);
            }
            else
            {
                fun(d,e,f,a,b,c);
            }

        circle.radius  = sqrt((circle.x-x1)*(circle.x-x1)+(circle.y-y1)*(circle.y-y1));
        printf("%.3lf %.3lf %.3lf\n",circle.x,circle.y,circle.radius);
    }
    return 0;
}

int fun(double a,double b,double c,double d,double e,double f)
{
    circle.y=(c*d-a*f)/(b*d-e*a);
    circle.x=(c-b*circle.y)/a;
    return 0;
}