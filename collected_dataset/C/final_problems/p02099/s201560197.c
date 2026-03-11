// AOJ 3001: GPA JANKEN
// 2017.11.29 bal4u@uu

#include <stdio.h>

int a[100002];
int p[100002];
int tr[4002];

char buf[10], *bp;
int getint() { int n = 0; while (*bp >= '0') n = (n<<3) + (n<<1) + (*bp++ & 0xf); return n; }
int getdbl() {
	int n = 0; while (*bp >= '0') n = (n<<3) + (n<<1) + (*bp++ & 0xf);
	if (*bp == '.') { bp++; while (*bp >= '0') n = (n<<3) + (n<<1) + (*bp++ & 0xf);	} return n;
}

int main()
{
	int n, i, j;

    fgets(bp=buf, 10, stdin), n = getint();
	for (i = 0; i < n; i++) {
		fgets(bp=buf, 10, stdin);
		a[i] = getdbl();
	}

    for (i = 0; i < n; i++) {
		if (tr[a[i]]) { p[i] = tr[a[i]]; continue; } 
        for (j = 0; j < n; j++) {
            if      (a[i]  > a[j]) p[i] += 3;
            else if (a[i] == a[j]) p[i]++;
        }
        tr[a[i]] = p[i];
    }
	for (i = 0; i < n; i++) printf("%d\n", p[i]-1);
    return 0;
}