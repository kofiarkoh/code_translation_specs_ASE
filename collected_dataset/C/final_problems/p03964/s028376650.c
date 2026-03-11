#include <stdio.h>
#include <stdlib.h>

int main(void){
    int n, i;
    int vote_tk = 0, vote_a = 0;
    int vote_tk_previous = 1, vote_a_previous = 1;
    long long int ans = 0;

    scanf("%d", &n);
    
    int *tp, *ap;
    tp = (int *)malloc(n * sizeof(int));
    ap = (int *)malloc(n * sizeof(int));

    for(i = 0; i < n; i++){
        scanf("%d %d", &tp[i], &ap[i]);
        for(int p = 1; !(vote_tk >= vote_tk_previous && vote_a >= vote_a_previous && vote_tk * ap[i] == vote_a * tp[i]); p++){
            vote_tk = tp[i] * p;
            vote_a = ap[i] * p;
        }
        //printf("current vote_tk = %d, vote_a = %d\n", vote_tk, vote_a);
        vote_tk_previous = vote_tk;
        vote_a_previous = vote_a;
    }

    ans = vote_tk + vote_a;
    printf("%lld\n", ans);
    free(tp);
    free(ap);
    return 0;
}