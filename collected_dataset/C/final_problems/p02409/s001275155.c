#define _CRT_SECURE_NO_WARNINGS
#include <stdio.h>
int main(void) {
    int n, b, f, r, v, a[4][3][10] = {0};
    scanf("%d", &n);
    for (int i = 0; i < n; i++) {
        scanf("%d%d%d%d", &b, &f,&r,&v);
        a[b-1][f-1][r-1]+=v;
        
    }
    for (int i = 0; i <= 3; i++) {
        for (int j = 0; j <= 2; j++) {
            for (int k = 0; k <= 9; k++) {
                printf(" %d", a[i][j][k]);
            }
            printf("\n");
        }
        if (i == 3)break;
        printf("####################\n");
    }
    return 0;
}
