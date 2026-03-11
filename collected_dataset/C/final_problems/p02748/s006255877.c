#include <stdio.h>
#include <stdlib.h>
#include <math.h>
int re,de,a[10000],b[10000],x[10000],y[10000],c[10000],n,m,i,j,k,sum,total,small,t,g,v;
//long long int ;
//float ;
//double ;
//char a[];
int main(void){
    scanf("%d %d %d",&re,&de,&m);
    for (i = 0; i < re; i++)
    {
        scanf("%d",&a[i]);
        k=a[i];
        if(i>0){
            if (t>a[i]){
                t=a[i];
            }
        }else
        {
            t=k;
        }
    }
    for (i = 0; i < de; i++)
    {
        scanf("%d",&b[i]);
        j=b[i];
        if(i>0){
            if (g>b[i]){
                g=b[i];
            }
        }else
        {
            g=j;
        }
        

    }
    for (i = 0; i < m; i++)
    {
        scanf("%d",&x[i]);
        scanf("%d",&y[i]);
        scanf("%d",&c[i]);
    }
    small=t+g;
    for (i = 0; i < m; i++)
    {
        if (a[x[i]-1]+b[y[i]-1]-c[i]<small)
        {
            small=a[x[i]-1]+b[y[i]-1]-c[i];
        }
    }
    printf("%d\n",small);
    return 0;
}