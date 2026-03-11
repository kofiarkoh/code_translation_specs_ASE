#include<stdio.h>
#define max(a,b) a>b?a:b
int a[1001][1001]={0};
int main(){
    int i,j,k,n,x1,x2,y1,y2,count=0;
    scanf("%d",&n);
    while(n--){
        scanf("%d%d%d%d",&x1, &y1, &x2, &y2);
        for(j=x1;j<x2;j++)for(k=y1;k<y2;k++)a[k][j]++,count=max(count,a[k][j]); 
    }
    printf("%d\n",count);
    return 0;
}