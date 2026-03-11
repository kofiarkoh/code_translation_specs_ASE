// AOJ 2190: Angel Stairs
// 2017.12.10

#include <stdio.h>
#include <string.h>

int n, m;
char t[50002], s[50002];
char tr['G'+1]['#'+1];
char buf[150005], *p;

int getint()
{
	int n = 0;
	while (*p >= '0') n = (n<<3) + (n<<1) + (*p++ & 0xf);
	p++;
	return n;
}

int check(int k)
{
	int i, j, x, nx, f;

    for (i = 0; i < m; i++) {
		if (k > n-1) return 0;
        x = s[i], f = 0;
        for (j = -1; j <= 1; j++) {
			nx = t[k] + j;
			if (nx < 0) nx += 12;
			else if (nx >= 12) nx -= 12;

            if (x == nx) {
                f = 1, k += j;
                if (j >= 0) k++;
                break;
            }
        }
        if (k < 0 || !f) return 0;
    }
    return k == n;
}

int main()
{
	int cno, i, a, b;

	tr['C'][0] = 0, tr['C']['#'] = 1, tr['D'][0] = 2, tr['D']['#'] = 3;
	tr['E'][0] = 4, tr['F'][0] = 5, tr['F']['#'] = 6;
	tr['G'][0] = 7, tr['G']['#'] = 8, tr['A'][0] = 9, tr['A']['#'] = 10;
    tr['B'][0] = 11;

    fgets(p=buf, 10, stdin), cno = getint();
	while (cno--) {
		fgets(p=buf, 20, stdin);
		n = getint(), m = getint();

		fgets(p=buf, sizeof(buf), stdin);
		for (i = n-1; i >= 0; i--) {
			a = *p++;
			b = *p++;
			if (b == '#') p++; else b = 0;
			t[i] = tr[a][b];
		}

		fgets(p=buf, sizeof(buf), stdin);
		for (i = m-1; i >= 0; i--) {
			a = *p++;
			b = *p++;
			if (b == '#') p++; else b = 0;
			s[i] = tr[a][b];
		}
		puts(check(0) || check(1) ? "Yes" : "No");
	}
	return 0;
}