// Aizu 1117: Missing Numbers
// 2017.9.15 bal4u@uu

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX 1000

typedef struct { int r, c, f; } T;
T q[MAX+5]; int len;
long long a[105][15]; char b[105][15]; int h, w;
long long sr[105], sc[15];
int kr[105], kc[15];

///=======================
// for topologicalSort
int  Q[MAX+5], top, end;
int  count[MAX+5];
char tbl[MAX+5][MAX+5];
void topologicalSort(int size)
{
	int i, j, r, c;

	memset(count, 0, sizeof(count));
	top = end = 0;
	for (i = 0; i < size; i++) for (j = 0; j < size; j++) if (tbl[i][j]) count[j]++;
	for (i = 0; i < size; i++) if (count[i] == 1) Q[end++] = i;
	while (top < end) {
		i = Q[top++];
		r = q[i].r, c = q[i].c;
		if (kc[r] == 1) {
			a[r][c] = a[r][w] - sc[r], b[r][c] = 0, sc[r] += a[r][c], sr[c] += a[r][c];
			kc[r] = 0, kr[c]--;
		} else {
			a[r][c] = a[h][c] - sr[c], b[r][c] = 0, sr[c] += a[r][c], sc[r] += a[r][c];
			kr[c] = 0, kc[r]--;
		}
		for (j = 0; j < size; j++) {
			if (tbl[i][j]) if (--count[j] == 1) Q[end++] = j;
		}
	}
}

int main()
{
	int r, c, i, j, k;
	char buf[20];
	int cno = 0;

	while (scanf("%d", &h) && h) {
		scanf("%d", &w); cno++;
		for (len = 0, r = 0; r <= h; r++) for (c = 0; c <= w; c++) {
			scanf("%s", buf);
			if (*buf == '?') a[r][c] = 0, b[r][c] = 1, q[len].r = r, q[len].c = c, q[len++].f = 1;
			else             a[r][c] = atoi(buf), b[r][c] = 0;
		}
		for (j = 0; j < 4; j++) {
			if (j == 0 || j == 3) for (r = 0; r < h; r++) {
				for (sc[r] = k = 0, c = 0; c < w; c++) {
					sc[r] += a[r][c], k += b[r][c]; if (b[r][c]) i = c;
				}
				if (k == 1)
					c = i, a[r][c] = a[r][w] - sc[r], b[r][c] = 0, sc[r] += a[r][c], sr[c] += a[r][c];
			}
			if (j == 1 || j == 2) for (c = 0; c < w; c++) {
				for (sr[c] = k = 0, r = 0; r < h; r++) {
					sr[c] += a[r][c], k += b[r][c]; if (b[r][c]) i = r;
				}
				if (k == 1)
					r = i, a[r][c] = a[h][c] - sr[c], b[r][c] = 0, sr[c] += a[r][c], sc[r] += a[r][c];
			}
		}
		for (k = 0, i = 0; i < len; i++) if (b[q[i].r][q[i].c]) k = 1; else q[i].f = 0;
		if (k) {
			for (r = 0; r < h; r++) { for (k = c = 0; c < w; c++) k += b[r][c]; kc[r] = k; }
			for (c = 0; c < w; c++) { for (k = r = 0; r < h; r++) k += b[r][c]; kr[c] = k; }
			memset(tbl, 0, sizeof(tbl));
			for (i = 0; i < len; i++) {
				if (!q[i].f) continue;
				for (j = i+1; j < len; j++) {
					if (!q[j].f) continue;
					tbl[i][j] = tbl[j][i] = 1;
				}
			}
			topologicalSort(len);
			for (k = 0, i = 0; i < len; i++) { if (b[q[i].r][q[i].c]) k = 1; break; }
		}
		if (cno > 1) putchar('\n');
		if (k == 0) for (i = 0; i < len; i++) printf("%lld\n", a[q[i].r][q[i].c]);
		else puts("NO");
    }
	return 0;
}