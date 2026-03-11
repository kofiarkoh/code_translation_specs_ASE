#include <stdio.h>

int main(void)
{
    long long sum = 0;
    int N, M;
    int P[200];
    int T[200];
    int X[200];

    scanf("%d",&N);//問題数
    for(int i=1; i<=N; i++) scanf("%d",&T[i]);//問題を解く時間
    scanf("%d",&M);//ドリンク数 
    for(int i=1; i<=M; i++) scanf("%d %d",&P[i], &X[i]);//何問目が何秒で解けるようになる 

    for(int i=1; i<=M; i++){
        sum=0;

        for(int j=1; j<=N; j++){
            if(P[i] == j){
                sum += X[i];
            } else {
                sum += T[j];
            }
        }

        printf("%lld\n",sum);
    }



    return 0;
}