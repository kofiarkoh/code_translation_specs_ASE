#include <stdio.h>
#include <math.h>

int main(void)
{
    double A, B, C, K;
    int i;
    double tmpA, tmpB, tmpC, seisuu, shousuu;
    scanf("%lf%lf%lf%lf", &A, &B, &C, &K);
//    printf("%f %f %f", A, B, C);

    if (fabsf(A-B)>1e+18){
        printf("Unfair\n");
    } else {
        shousuu = modf(K/2.0,&seisuu);
//        printf("%f\n", shousuu);
        printf("%.0f\n", fabsf(shousuu-0.5)>1e-3 ? A-B : B-A );
    }
    return 0;
}