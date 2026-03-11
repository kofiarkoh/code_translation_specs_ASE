#include<stdio.h>

int n,W;
int w[100],v[100];

int max(int a, int b) {
    if (a < b) {
        return b;
    } else {
        return a;
    }
}

int rec(int i, int j) {
    int res;
    if (i == n) {
        res = 0;
    } else if (j < w[i]) {
        res = rec(i + 1, j);
    } else {
        res = max(rec(i + 1, j), rec(i + 1, j - w[i]) + v[i]);
    }
    return res;
}


int main(void) {
    scanf("%d%d",&n,&W);
    int i;
    for (i=0; i<n; i++) {
        scanf("%d%d",&w[i],&v[i]);
    }
    printf("%d\n",rec(0,W));
    return 0;
}
