#include <stdio.h>
#define REP(i, n) for(i = 0; i < n; i++)
#define FOR(i, a, b) for(i = a; i < b; i++)
#define N 100000
#define sqN 397  //floor(sqrt(N)) + 1

void is_prime(char num[N + 1]);

int main(){
    char num[N + 1]; //num[i]にはiが素数かどうか(1, 0)が入る.
    int i;

    REP(i, N + 1){
        if(i % 2 == 0 && i > 2){
            num[i] = 0;
        }else{
            num[i] = 1;
        }
    }
    num[0] = 0;
    num[1] = 0;

    is_prime(num);

    int Q, l, r;
    int a, b;

    scanf("%d", &Q);
    REP(a, Q){
        scanf("%d %d", &l, &r);
        int cnt = 0;

        FOR(b, l, r + 1){
            if(b % 2 && num[b] == 1 && num[(b + 1) / 2] == 1){
                cnt++;
            }
        }

        printf("%d\n", cnt);
    }

    return 0;
}

void is_prime(char num[N + 1]){
    int x, k;

    FOR(x, 3, sqN + 1){
        if(num[x] == 1){
            FOR(k, 2, N / x + 1){
                if(k * x < N + 1) num[k * x] = 0;
            }
        }
    }
}