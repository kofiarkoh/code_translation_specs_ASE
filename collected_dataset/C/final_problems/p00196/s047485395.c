#include<stdio.h>
int main(void)
{
    int n, i, t, r, j, temp;
    char str[11] = { 0 };
    char a[10] = { 0 };
    int b[10 + 1][10 + 1];
    int c[10 + 1];
    int d[10 + 1];
    int hideo[10 + 1];
    scanf("%d", &n);
    while (n > 2 && n<10) {
        i = t = r = j = temp = 0;
        for (i = 1; i <= n; i++) {
            c[i] = 0;
            d[i] = 0;
            hideo[i] = 0;
            for (j = 1; j <= n; j++) {
                b[i][j] = 0;
            }
        }

        for (i = 1; i <= n; i++) {
            scanf("%s", &a[i]);
            str[i] = a[i];
            for (j = 1; j <= n - 1; j++) {
                scanf("%d", &b[i][j]);
                if (b[i][j] == 0) {
                    c[i]++;

                }
                if (b[i][j] == 1) {
                    d[i]++;
                }
            }
        }
        for (i = 1; i <= n; i++) {
            hideo[i] = c[i];
        }
        for (i = 1; i < n; i++) {
            for (j = n; j > i; j--) {
                if (c[j] > c[j - 1]) {
                    temp = c[j];
                    c[j] = c[j - 1];
                    c[j - 1] = temp;
                    a[1] = str[j];
                    str[j] = str[j - 1];
                    str[j - 1] = a[1];



                }
            }
        }
        for (i = 1; i <= n; i++) {
            if (c[i] == c[i + 1]) {
                if (d[i] > d[i + 1]) {
                    temp = c[i];
                    c[i] = c[i + 1];
                    c[i + 1] = temp;
                }
            }
        }
        for (i = 1; i <= n; i++) {
            printf("%c\n", str[i]);
        }
        scanf("%d", &n);
    }
}