#include<stdio.h>

int main()
{

    int n,p,i;
    long long
    int x[4],y[4],s1x,s2x,s1y,s2y;
    scanf("%d",&n);
    while(n--){
        for(i=0;i<4;i++)
            scanf("%lld %lld",&x[i],&y[i]);
        s2x=x[3]-x[2];
        s1x=x[1]-x[0];
        s2y=y[3]-y[2];
        s1y=y[1]-y[0];
        if(s1y*s2y + s1x*s2x==0)
            printf("1\n");
        else if(s1x*s2y - s1y*s2x==0)
            printf("2\n");

        else
            printf("0\n");




    }

    return 0;

}

