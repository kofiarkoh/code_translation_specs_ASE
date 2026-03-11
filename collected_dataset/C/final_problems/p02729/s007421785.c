#include<stdio.h>
int main()
{
    int j,i,f,e=1,d=1,N,M,s=0,h,g;
    scanf("%d%d",&N,&M);
    h=N;
    for(i=h;i>N-2;i--)
    {
        d=i*d;
    }
    g=M;
    for(j=g;j>M-2;j--)
    {
        e=j*e;
    }
            f=(e+d)/2;
    printf("%d",f);
}
