#include <stdio.h>
#include <string.h>

int main(void) {
    char S[500005];
    int a[500005], b[500005], N, i, x, r, s, M;
    scanf("%s", S);
    N=strlen(S);
    if (S[0]=='<') {
        r=0; x=1;
    }
    else {
        b[1]=0; r=0; x=2;
    }
    for (i=1; i<N; i++) {
        if (S[i]!=S[i-1]) {
            b[x]=r+1;
            r=0; x++;
            // printf("%d ", r);
            continue;
        }
        r++;
        // printf("%d ", r);
    }
    b[x]=r+1;
    if (x%2==1) {
        x++;
        b[x]=0;
    }
    // printf("\n");
    // printf("%s\n", S);
    // for (i=1; i<=x; i++) printf("%d ", b[i]);
    // printf("\n");
    s=0;
    for (i=1; i<=x/2; i++) {
        if (b[2*i-1]>b[2*i]) M=b[2*i-1];
        else M=b[2*i];
        // printf("%d ", b[2*i-1]*(b[2*i-1]-1)/2+b[2*i]*(b[2*i]-1)/2+M);
        s+=b[2*i-1]*(b[2*i-1]-1)/2+b[2*i]*(b[2*i]-1)/2+M;
    }
    // printf("\n");
    printf("%d\n", s);
    return 0;
}