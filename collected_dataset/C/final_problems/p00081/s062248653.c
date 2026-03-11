#include<stdio.h>
 
int main()
{
    double x1,y1,x2,y2,xq,yq,xr,yr,m,n;
    for(;~scanf("%lf,%lf,%lf,%lf,%lf,%lf",&x1,&y1,&x2,&y2,&xq,&yq);)
    {
        if(x2==x1)
            printf("%f %f\n",2*x1-xq,yq);
        else if(y2==y1)
            printf("%f %f\n",xq,2*y1-yq);
        else
        {
            x2-=x1;y2-=y1;xq-=x1;yq-=y1;
            m=y2/x2;
            n=1/(1+m*m);
            xr=n*(1-m*m)*xq+2*m*n*yq+x1;
            yr=n*2*m*xq+n*(m*m-1)*yq+y1;
            printf("%f %f\n",xr,yr);
        }
    }
    return 0;
}