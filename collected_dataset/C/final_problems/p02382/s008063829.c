#include <stdio.h>
#include <stdlib.h>
#include <math.h>
 
int main(){
    int n, x[100], y[100], i, p;
    double sum, max = 0;
     
    scanf("%d", &n);
    for(i=0; i<n; i++) scanf("%d", &x[i]);
    for(i=0; i<n; i++) scanf("%d", &y[i]);
     
    for(p=1; p<=3; p++){
        sum = 0;
        for(i=0; i<n; i++){
            sum += pow(fabs(x[i]-y[i]), p);
        }
        switch(p){
        case 2:
            sum = sqrt(sum);
            break;
        case 3:
            sum = cbrt(sum);
            break;
        }
        printf("%f\n", sum);
    }
     
    for(i=0; i<n; i++){
        if(fabs(x[i]-y[i]) > max) max = fabs(x[i]-y[i]);
    }
    printf("%f\n", max);
     
    return 0;
}