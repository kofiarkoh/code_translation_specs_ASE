#include <stdio.h>
#define max(X, Y) ((X)>(Y)?(X):(Y))

double p[32][10010];
int main(void)
{
    int N, M, K;
    while (scanf("%d%d%d", &N, &M, &K), N) {
        int i, j, k;
        double res;
        for(i=0;i<=N;++i) for(j=0;j<=N*M;++j) p[i][j]=0;
        p[0][0]=1;
        for(i=1;i<=N;++i) {
            for(j=1;j<=N*M;++j) {
                p[i][j]=0;
                for(k=1;k<=M;++k) if (j>=k) p[i][j]+=p[i-1][j-k];
                p[i][j]/=M;
            }
        }
        res=0;
        for(i=1;i<=N*M;++i) res+=max(i-K, 1)*p[N][i];
        printf("%.8f\n", res);
    }
    return 0;
}