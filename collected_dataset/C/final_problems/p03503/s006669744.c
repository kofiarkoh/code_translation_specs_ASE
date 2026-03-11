#include<stdio.h>
int main()
{
    int N;
    int F[100][5][2];
    int P[100][11];
    int i,j,k,flag1,flag2;
    int counT=0,profit=0,maX=0;
    scanf("%d",&N);
    for(i=0;i<N;i++)
        for(j=0;j<5;j++)
            scanf("%d %d",&F[i][j][0],&F[i][j][1]);
    for(i=0;i<N;i++)
        for(j=0;j<11;j++)
            scanf("%d",&P[i][j]);
    for(i=0,flag2=0;i<N;i++)
    {
        for(j=0,counT=0;j<5;j++)
            for(k=0;k<2;k++)
                if(F[i][j][k]==1)
                   counT++;
        for(j=1,flag1=0;j<=counT;j++)
        {
            if(P[i][0]<=P[i][j])
                flag1=1;
        }
        if(flag1==0)
            flag2++;
    }
    if(flag2!=N)
    {
        for(i=0;i<N;i++)
        {
            for(j=0,counT=0;j<5;j++)
                for(k=0;k<2;k++)
                   if(F[i][j][k]==1)
                      counT++;
            for(j=0,profit=P[i][0];j<=counT;j++)
                if(profit<P[i][j])
                   profit=P[i][j];
            maX+=profit;
        }
    }
    else
    {
        for(i=0,profit=P[0][1];i<N;i++)
           for(j=1;j<=counT;j++)
                if(profit<P[i][j])
                    profit=P[i][j];
        maX=profit;
    }
    printf("%d",maX);
}