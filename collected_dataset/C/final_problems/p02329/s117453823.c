#include <stdio.h>
#define N 1000
int main(void){
    int i,j,k,l;
    int n;
    long long int v;
    long long int num[4][N];
    long long int min[4],max[4];
    int ans=0;
    scanf("%d%lld",&n,&v);
    for(i=0;i<4;i++)
    {
        for(j=0;j<n;j++)
        {
            scanf("%lld",&num[i][j]);
            if(j==0) min[i]=max[i]=num[i][j];
            if(num[i][j]<min[i]) min[i]=num[i][j];
            else if(num[i][j]>max[i]) max[i]=num[i][j];
        }
    }
    for(i=0;i<n;i++)
    {
        if(num[0][i]>v-min[3]-min[2]-min[1]||num[0][i]<v-max[3]-max[2]-max[1]) continue;
        for(j=0;j<n;j++)
        {
            if(num[0][i]+num[1][j]>v-min[3]-min[2]||num[0][i]+num[1][j]<v-max[3]-max[2]) continue;
            for(k=0;k<n;k++)
            {
                if(num[0][i]+num[1][j]+num[2][k]>v-min[3]||num[0][i]+num[1][j]+num[2][k]<v-max[3]) continue;
                for(l=0;l<n;l++)
                {
                    if(num[0][i]+num[1][j]+num[2][k]+num[3][l]==v) ans++;
                }
            }
        }
    }
    printf("%lld\n",ans);
    return 0;
}
