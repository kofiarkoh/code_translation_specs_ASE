// AOJ 2555 Everlasting Zero
// 2018.4.25 bal4u

#include <stdio.h>
#include <string.h>

// バッファを経ずstdinから数値を得る
//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0, c = getchar_unlocked();
	while (c <= ' ') c = getchar_unlocked();
	do n = 10*n + (c & 0xf), c = getchar_unlocked(); while (c >= '0');
	return n;
}
 
#define INF 110
char min[101][101], max[101][101];
char to[101][101], hi[101];
int q[101], top, end;
int count[101];

int topological_sort(int n)
{
    int i, j, k, f;

//  for (i = 0; i < n; i++) for (j = 0; j < hi[i]; j++) count[to[i][j]]++;
    top = end = 0, f = 0;
    for (i = 0; i < n; i++) if (!count[i]) q[end++] = i;
    while (top < end) {
		i = q[top++];
        f++;
        for (j = 0; j < hi[i]; j++) {
            k = to[i][j];
            if (--count[k] == 0) q[end++] = k;
        }
    }
	return f == n;
}

int main()
{
	int M, N, K, s, cond, t, i, j, k;

	M = in(), N = in();
	memset(max, INF, sizeof(max));
	for (i = 0; i < M; i++) {
		K = in();
		while (K--) {
			s = in()-1;
			cond = (getchar_unlocked() == '<') ;
			getchar_unlocked(), getchar_unlocked();
			t = in();
			if (cond) {
				if (t < max[i][s]) max[i][s] = t;
			} else {
				if (t > min[i][s]) min[i][s] = t;
			}
			if (min[i][s] > max[i][s]) { puts("No"); return 0; }
		}


	}
	for (i = 0; i < N; i++) {
		for (j = 0; j < M; j++) for (k = 0; k < M; k++) if (j != k) {
			if (min[j][i] > max[k][i]) to[j][hi[j]++] = k, count[k]++;
		}
	}
	puts(topological_sort(M)? "Yes": "No");
	return 0;
}
