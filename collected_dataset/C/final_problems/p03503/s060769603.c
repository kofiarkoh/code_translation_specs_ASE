#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<stdint.h>
#include<tgmath.h>
#include<stdbool.h>
#include<sys/queue.h>
#include<time.h>
#define ll long long
#define min(a, b) ((a) > (b) ? (b) : (a))
#define max(a, b) ((a) > (b) ? (a) : (b))

int N, F[111][10], P[111][11], ans = -1001001001;
int main(){
    scanf("%d", &N);
    for(int i=0; i<N; i++){
        for(int j=0; j<10; j++)scanf("%d", &F[i][j]);
    }
    for(int i=0; i<N; i++){
        for(int j=0; j<=10; j++)scanf("%d", &P[i][j]);
    }
    for(int i=1; i<1024; i++){
        int profit = 0;
        for(int j=0; j<N; j++){
            int cnt = 0;
            for(int k=0; k<10; k++){
                if(i&(1<<k))cnt += F[j][k];
            }
            profit += P[j][cnt];
        }
        ans = max(ans, profit);
    }
    printf("%d\n", ans);
    return 0;
}