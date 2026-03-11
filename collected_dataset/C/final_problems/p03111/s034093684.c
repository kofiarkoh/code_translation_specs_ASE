#include <stdio.h>
#include <stdlib.h>
#define min(X, Y) ((X) < (Y) ? (X) : (Y))

const int inf = 1 << 30;
int a, b, c;
int n;
int lll[5];
int ll[5];
int l[10];


int solve(int pos)
{
    if (pos >= n) {
        int res = 0;
        for (int i = 0; i < 3; i++) {
            if (ll[i] == 0) {
                res = inf;
            }
            else {
                res += abs(ll[i] - lll[i]);
            }
        }

        return res;
    }

    int res = inf;
    for (int i = 0; i < 4; i++) {
 
        if (ll[i] == 0 || i == 3) {
            ll[i] = l[pos];
            res = min(solve(pos + 1), res);
        }
        else {
            ll[i] += l[pos];
            res = min(solve(pos + 1) + 10, res);
        }
        ll[i] -= l[pos];
    }

    return res;
}

int main()
{
    scanf("%d %d %d %d", &n, lll, lll + 1, lll + 2);
    for (int i = 0; i < n; i++) {
        scanf("%d", l + i);
    }

    printf("%d\n", solve(0));

    return 0;
}
