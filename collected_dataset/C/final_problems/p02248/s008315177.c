#include <stdio.h>
#include <string.h>

#define MAX(a,b) ((a)>(b) ? (a):(b))

char T[1000010];
char P[10010];
int skip[128];

void solve()
{
    int i, j;
    int plen = strlen(P);
    int tlen = strlen(T);
        
    for (i=0; i<128; i++) {
        skip[i] = plen;
    }
    for (i=0; i<plen; i++) {
        skip[P[i]] = plen - i - 1;
    }
    
    /*
    // i+=skip[P[j]]??§??????????????¨?????¨???
    for (i=plen-1; i<tlen; i+=MAX(skip[P[j]], plen - j)) {
        for (j=plen-1; (j>=0 && T[i]==P[j]); i--,j--) {
            if (j == 0) {
                printf("%d\n", i);
                break;
            }
        }
    }
    */
    
    i = plen - 1;
    while (i < tlen) {
        j = plen - 1;
        while (j >= 0) {
            if (T[i] != P[j]) {
                break;
            }
            if (j == 0) {
                printf("%d\n", i);
                break;                
            }
            i--;
            j--;
        }
        i += MAX(skip[P[j]], plen - j);
    }
}


int main()
{
    scanf("%s", T);
    scanf("%s", P);
    
    solve();
    
    return 0;
}