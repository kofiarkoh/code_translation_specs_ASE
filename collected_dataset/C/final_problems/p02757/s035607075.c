#include <stdio.h>
#include <stdlib.h>

int A[10000];
int main() {
    //printf("%", atoi("0123"));
    int n, p;
    scanf("%d %d", &n, &p);
    char s[n+1];
    scanf("%s", s);
    int count = 0;
    if (p == 2 || p == 5) {
        for (int i = 0; i < n; i++) {
            int d = s[i] - '0';
            if ((d % 2 == 0) || (d % 5 == 0)) {
                count += i + 1;
            }
        }
    } else {
        int mods[n+1];
        mods[n] = 0;
        A[0] = 1;
        int mod10 = 1;
        for (int i = n - 1; i >= 0; i--) {
            mod10 = i == n - 1 ? 1 : ((mod10 * 10) % p);
            mods[i] = ((s[i] - '0') * mod10 + mods[i+1]) % p;
            count += A[mods[i]];
            A[mods[i]]++;
            //printf("%d,", mods[i]);
        }
    }
    printf("%d", count);
    return 0;
}