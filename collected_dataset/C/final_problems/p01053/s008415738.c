#include<stdio.h>
long long d[5010]={1};
int p[5010][5010]={1,0};
int main(){
    int M=1000000007;
    int n,m,k,q,a,b,c,i,j,r;
    scanf("%d %d %d %d",&n,&m,&k,&q);
    for(i=1;i<=m;i++){
        for(j=r=0;j<k;j++){
            r+=p[i-1][(2*k-j)%(2*k)];
            r%=M;
        }
        for(j=0;j<k*2;j++){
            p[i][j]=r;
            r+=p[i-1][(j  +1)%(2*k)];
            r-=p[i-1][(j+k+1)%(2*k)];
            r%=M;
            p[i][j]%=1000000007;//printf("%d ",p[i][j]);
        }
        d[i]=d[i-1]*(n/2/k);
        d[i]%=1000000007;//printf(":%lld\n",d[i]);
    }
    while(q--){
        scanf("%d %d %d",&a,&b,&c);
        printf("%lld\n",(p[b][(c-a+2*k)%(2*k)]*d[b-1])%1000000007);
    }
    return 0;
}