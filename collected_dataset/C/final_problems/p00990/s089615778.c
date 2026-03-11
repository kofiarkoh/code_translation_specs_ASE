// Aizu Vol-15 1500: ID
// 2017.9.4 bal4u@uu

#include <stdio.h>
#include <string.h>

#define N       1000000000
#define MAXINT  50

void mpNum2Str(char *str, int *num)
{
    int  i, j;
    char *ss;
    int  x;

    if (*num == 0 || (*num == 1 && *(num+1) == 0)) {
        *str++ = '0';
        *str = '\0';
        return;
    }

    ss = str - 1;
    for (i = *num; i > 0; i--) {
        x = *++num;
        for (j = 1; j < N; j *= 10) {
            *++ss = x % 10 + '0';
            x /= 10;
        }
    }
    while (*ss == '0') ss--;

    *(ss + 1) = '\0';    
    while (str < ss) {
        x = *str;
        *str++ = *ss;
        *ss-- = x;
    }
}

void mpAdd(int *ret, int *a, int *b)
{
    int  i;
    int  lr, la, lb;
    int  *rr;
    int  x;

    la = *a;
    lb = *b;
    lr = (la >= lb) ? la : lb;

    rr = ret;
    x = 0;
    for (i = 1; i <= lr; i++) {
        if (i <= la) x += *++a;
        if (i <= lb) x += *++b;
        if (x < N) {
            *++rr = x;
            x = 0;
        } else {
            *++rr = x - N;
            x = 1;
        }
    }
    *++rr = x;
    *ret = lr + x;
}

void mpCpy(int *des, int *src)
{
    memcpy(des, src, (1+*src)*sizeof(int));
}

char id[100003];
char a[10];
int t[10] = { 0,2,4,6,8,1,3,5,7,9 };

int p[2][10][MAXINT], tmp[MAXINT];
char buf[510];

int main()
{
	int n, m, i, j, d, x, k, k1, k2, even;

	scanf("%d%s%d", &n, id+1, &m);
	for (i = 0; i < m; i++) scanf("%d", &d), a[d] = 1;
	p[0][0][0] = p[0][0][1] = 1, even = !(n & 1);
	for (k1 = 0, k2 = 1, i = 1; i <= n; i++, k1 = k2, k2 = !k2, even = !even) {
		for (j = 0; j < 10; j++) p[k2][j][0] = 0;
		if (id[i] == '*') {
			for (k = 0; k < 10; k++) {
				if (!a[k]) continue;
				d = even ? t[k] : k;
				for (j = 0; j < 10; j++) {
					if ((x = d + j) >= 10) x -= 10;
					mpAdd(tmp, p[k2][x], p[k1][j]);
					mpCpy(p[k2][x], tmp);
				}
			}
		} else {
			k = id[i] - '0';
			d = even ? t[k] : k;
			for (j = 0; j < 10; j++) {
				if ((x = d + j) >= 10) x -= 10;
				mpAdd(tmp, p[k2][x], p[k1][j]);
				mpCpy(p[k2][x], tmp);
			}
		}
	}
	mpNum2Str(buf, p[k1][0]);
	puts(buf);
	return 0;
}