#include<stdio.h>
#define max(a,b) a>b?a:b
#define min(a,b) a<b?a:b
int a[1001][1001];
int main(){
    int i,j,n,x1,x2,y1,y2,x1_min=1000,y1_min=1000,x2_max=0,y2_max=0,count=0;
    scanf("%d",&n);
    while(n--){
        scanf("%d%d%d%d",&x1, &y1, &x2, &y2);
        a[x1][y1]++,a[x1][y2]--;a[x2][y1]--,a[x2][y2]++;
        x1_min=min(x1,x1_min),x2_max=max(x2,x2_max),y1_min=min(y1,y1_min),y2_max=max(y2,y2_max);
    }
    for(i=x1_min;i<=x2_max;i++)for(j=y1_min;j<=y2_max;j++){
        if(i)a[i][j]+=a[i-1][j];
        if(j)a[i][j]+=a[i][j-1];
        if(i&&j)a[i][j]-=a[i-1][j-1];
        count=max(count,a[i][j]);
    }
    printf("%d\n",count);
    return 0;
}