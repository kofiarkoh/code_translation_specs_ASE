#include <stdio.h>

#define FOR(i, a, b) for(i = a; i < b; ++i)
#define REP(i, n) FOR(i, 0, n)
#define comp(a, b) comp(const void *a, const void *b)
#define MOD 1000000007
typedef unsigned int uint;
typedef unsigned long ulong;
typedef unsigned long long ull;

void yes_or_no(char boolean){
    if(boolean){
        printf("Yes\n");
    }else{
        printf("No\n");
    }
}
ull pow2(uint k){
    if(k == 0){
        return 1;
    }else{
        return (2 * pow2(k - 1)) % MOD;
    }
}
int main(){
    uint N, i;
    uint A;
    scanf("%d", &N);
    char flag = 1;

    if(N % 2 == 1){
        uint num[N/2 + 1];
        REP(i, N/2 + 1){
            num[i] = 0;
        }

        REP(i, N){
            scanf("%d", &A);
            if(A % 2 == 1 || A > N - 1){
                flag = 0;
                break;
            }
            else{
                num[A/2]++;
                if(A == 0 && num[A/2] > 1){
                    flag = 0;
                    break;
                }else if(num[A/2] > 2){
                    flag = 0;
                    break;
                }
            }
        }

        REP(i, N/2 + 1){
            if(i == 0 && num[i] != 1){
                flag = 0;
                break;
            }else if(i > 0 && num[i] != 2){
                flag = 0;
                break;
            }
        }
    }else{
        uint num[N/2];
        REP(i, N/2){
            num[i] = 0;
        }
        REP(i, N){
            scanf("%d", &A);
            if(A % 2 == 0 || A > N - 1){
                flag = 0;
                break;
            }
            else{
                num[A/2]++;
                if(num[A/2] > 2){
                    flag = 0;
                    break;
                }
            }
        }

        REP(i, N/2){
            if(num[i] != 2){
                flag = 0;
                break;
            }
        }
    }

    if(flag == 0){
        printf("0\n");
    }else{
        printf("%lld\n", pow2(N/2));
    }

    return 0;
}