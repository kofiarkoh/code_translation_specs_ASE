#include <stdio.h>
#include <stdlib.h>
#include <math.h>
int main(void)
{
    int N, D;
    double **X;
    double r;
    int i,j,k; 
    scanf("%d %d", &N, &D);
    X = malloc(sizeof(double *)*N);
    for(i=0;i<N;i++)
        X[i] = malloc(sizeof(double)*D);
    for(i=0; i < N; i++){
        for(j=0; j < D; j++){
            scanf("%lf", &r);
            X[i][j] = r;
        }
    } 
    int count=0;
    double dist;
    for(i=0; i<N; i++){
        for(k=0; k<N; k++){
            if(i==k)continue;
            dist = 0.0;
            for(j=0; j<D; j++){
                dist += (X[i][j]-X[k][j])*(X[i][j]-X[k][j]);
            }
            dist = sqrt(dist);
            double ch = dist - (double)((int)dist);
            if(ch==0.0)
                count+=1;
        }
    }
    printf("%d\n", count/2);
    free(X);
    return 0;
}