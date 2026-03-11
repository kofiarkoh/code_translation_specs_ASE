#include <stdio.h>

int main(void)
{
    int N, t;
    int x[1001], h[1001];
    double max;
    double t2;
    int i;
    
    scanf("%d%d", &N, &t);
    for (i = 0; i < N; i++){
        scanf("%d%d", &x[i], &h[i]);
    }
    
    max = (double)h[0] / (double)x[0];
    for (i = 1; i < N; i++){
        if (t > x[i]){
            t2 = (double)h[i] / (double)x[i];
            if (max < t2){
                max = t2;
            }
        }
    }
    
    printf("%lf\n", max * t);
    
    return (0);
}

