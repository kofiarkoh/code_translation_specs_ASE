#include<stdio.h>
typedef long long ll;
#define int ll
ll p[5010][5010]={1,0};
ll d[5010]={1,1};
signed main(){
    int M=1000000007;
    int n,m,k,q,a,b,c,i,j;
    ll r;
    scanf("%lld %lld %lld %lld",&n,&m,&k,&q);
    for(i=1;i<=m;i++){
        for(j=r=0;j<k;j++){
            r+=p[i-1][(2*k-j)%(2*k)];
            r%=M;
        }
        for(j=0;j<k*2;j++){
            p[i][j]=r*n/(2*k)%M;
            if(i==1&&p[i][j])p[i][j]=1;;
            r+=p[i-1][(j  +1)%(2*k)];
            r-=p[i-1][(j+k+1)%(2*k)];
            r%=M;
            //printf("%lld ",p[i][j]);
        }//printf("\n");
        //d[i+1]=d[i]*n/(2*k);
        //d[i+1]%=M;printf(":%lld\n",d[i]);
    }
    while(q--){
        scanf("%lld %lld %lld",&a,&b,&c);
        printf("%lld\n",(p[b][(c-a+2*k)%(2*k)])%M);
    }
    return 0;
}