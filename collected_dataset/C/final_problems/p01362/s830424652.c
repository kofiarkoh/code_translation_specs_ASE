#include<stdio.h>
char dice[8][5][5];
int i,j,res,f[3];
int MIN(int x,int y){ return x<y?x:y; }
int input()
{
    for(i=1;i<=6;i++)
        for(j=2;j>=0;j--)
        {
            scanf("%s",dice[i][j]);
            if(i==1&&j==2&&dice[i][j][0]=='#') return 0;
        }
    return 1;
}
void f1(int a,int b,int m)
{
    for(f[m]=i=0;i<3;i++) if(dice[a][b][i]=='*') f[m]=1;
}
void f3(int a,int b,int m)
{
    for(f[m]=i=0;i<3;i++) if(dice[a][i][b]=='*') f[m]=1;
}
void C(int x)
{
    if(f[1]&&f[2]) res=MIN(res,x);
}
int main()
{
    while(input())
    {
        res=9;

        f1(1,0,1),f1(3,0,2),C(0);
        f1(1,2,1),f1(3,2,2),C(2);
        f3(1,0,1),f3(3,2,2),C(1);
        f3(1,2,1),f3(3,0,2),C(1);

        f1(2,0,1),f1(4,0,2),C(3);
        f1(2,2,1),f1(4,2,2),C(3);
        f3(2,0,1),f3(4,2,2),C(2);
        f3(2,2,1),f3(4,0,2),C(2);

        f1(5,0,1),f1(6,2,2),C(1);
        f1(5,2,1),f1(6,0,2),C(1);
        f3(5,0,1);
        for(f[2]=i=0;i<3;i++) if(dice[6][2-i][0]=='*') f[2]=1;
        C(2);
        f3(5,2,1);
        for(f[2]=i=0;i<3;i++) if(dice[6][2-i][2]=='*') f[2]=1;
        C(2);

        printf("%d\n",res);
    }
}