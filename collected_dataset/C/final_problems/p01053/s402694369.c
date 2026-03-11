#include<stdio.h>
long long p[5010][5010]={1,0};
long long d[5010]={1,1};
int main(){
    int M=1000000007;
    int n,m,k,q,a,b,c,i,j;
    long long r;
    scanf("%d %d %d %d",&n,&m,&k,&q);
    for(i=1;i<=m;i++){
        for(j=r=0;j<k;j++){
            r+=p[i-1][(2*k-j)%(2*k)];
            r%=M;
        }
        for(j=0;j<k*2;j++){
            p[i][j]=r%M;
            r+=p[i-1][(j  +1)%(2*k)];
            r-=p[i-1][(j+k+1)%(2*k)];
            r%=M;
            //printf("%lld ",p[i][j]);
        }//printf("\n");
        d[i+1]=d[i]*n/(2*k);
        d[i+1]%=M;//printf(":%lld\n",d[i]);
    }
    while(q--){
        scanf("%d %d %d",&a,&b,&c);
        printf("%lld\n",(p[b][(c-a+2*k)%(2*k)]*d[b])%M);
    }
    return 0;
}