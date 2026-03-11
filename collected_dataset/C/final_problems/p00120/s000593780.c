#include <stdio.h>
#include <string.h>
#include <math.h>

#define SQ(x) ((x) * (x))
#define INF (100000000)
#define EPS (1e-5)

double len;
double cake[12];
double dp[1 << 12][12];
int n;

double min(double a, double b)
{
    return (b > a ? a : b);
}

double tsp(int bit, int now)
{
    int i;
    double res;
    
    if (dp[bit][now] >= 0){
        return (dp[bit][now]);
    }
    
    if (bit == (1 << n) - 1 && now == 0){
        return (dp[bit][now] = 0);
    }
    
    res = INF;
    for (i = 0; i < n; i++){
        if (!((bit >> i) & 1)){
            res = min(res, tsp(bit | (1 << i), i) + sqrt(SQ(cake[i] + cake[now]) - SQ(cake[i] - cake[now])));
        }
    }
    
    return (dp[bit][now] = res);
}

int main(void)
{
    int i, j;
    
    while (scanf("%lf", &len) != EOF){
        getchar();
        n = 0;
        while (1){
            scanf("%lf", &cake[n++]);
            
            if (getchar() == '\n'){
                break;
            }
        }
        
        for (i = 0; i < (1 << 12); i++){
            for (j = 0; j < 12; j++){
                dp[i][j] = -1 * INF;
            }
        }
        
        if (tsp(0, 0) < len + EPS){
            printf("OK\n");
        }
        else {
            printf("NA\n");
        }
    }
    
    return (0);
}