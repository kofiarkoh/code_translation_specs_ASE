#include<stdio.h>
int main()
{
    int i,j,h,l;long long int a[500][500]={0};
    scanf("%d %d",&h,&l);
    {
        a[1][1]=1;
        a[1][2]=1;
        for(i=3;i<=l;i++)
            a[1][i]=a[1][i-1]+a[1][i-2];

        for(i=2;i<=h;i++)
            for(j=1;j<=l;j++)
            a[i][j]=a[i-1][j]+a[i][j-1];
     a[h][l]=a[h][l]%998244353;
        printf("%lld\n",a[h][l]);
    }
    return 0;
}
