#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

int main()
{
    double N,T,A;
    double H[1050];
    int ans;
    double min_diff = 100000;
    scanf("%lf%lf%lf",&N,&T,&A);
    for(int i=0;i<N;i++){
        scanf("%lf",&H[i]);
        if( abs( T - ( H[i] * 0.006 ) - A) < min_diff ){
            ans = i + 1;
            min_diff = abs( T - ( H[i] * 0.006 ) - A);
        }
    }

    printf("%d\n",ans);

    return 0;
}