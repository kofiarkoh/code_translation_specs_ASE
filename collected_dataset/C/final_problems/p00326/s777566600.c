// AOJ 0331 Scheduler
// 2018.2.18 bal4u

#include <stdio.h>
#include <string.h>
#include <stdlib.h>

typedef long long ll;
#define QSIZE 50000

#define PARENT(i) ((i)>>1)
#define LEFT(i)   ((i)<<1)
#define RIGHT(i)  (((i)<<1)+1)

typedef struct { int id; ll k1; int k2; } QUE;
QUE que[24][QSIZE+5];
int qsize[24];

void max_heapify(int qid, int i)
{
	int l, r, max;
	QUE qt;
	l = LEFT(i), r = RIGHT(i);
	if (l < qsize[qid] && (que[qid][l].k1 > que[qid][i].k1 ||
		que[qid][l].k1 == que[qid][i].k1 && que[qid][l].k2 > que[qid][i].k2)) max = l; else max = i;
	if (r < qsize[qid] && (que[qid][r].k1 > que[qid][max].k1 ||
		que[qid][r].k1 == que[qid][max].k1 && que[qid][r].k2 > que[qid][max].k2)) max = r;
	if (max != i) {
		qt = que[qid][i], que[qid][i] = que[qid][max], que[qid][max] = qt;
		max_heapify(qid, max);
	}
}

int deq(int qid)
{
	int id = que[qid][0].id;
//	if (qsize == 0) return 0;
	que[qid][0] = que[qid][--qsize[qid]];
	max_heapify(qid, 0);
	return id;
}

void enq(int qid, int id, ll k1, int k2)
{
	int i, max;
	QUE qt;

	i = qsize[qid]++;
	que[qid][i].id = id, que[qid][i].k1 = k1, que[qid][i].k2 = k2;
	while (i > 0 && (que[qid][max = PARENT(i)].k1 < que[qid][i].k1 ||
		que[qid][max].k1 == que[qid][i].k1 && que[qid][max].k2 < que[qid][i].k2)) {
		qt = que[qid][i]; que[qid][i] = que[qid][max], que[qid][max] = qt;
		i = max;
	}
}

#define MAX 50002

int N, K;
int sz[5] = {1,1,2,6,24};
int q[MAX], top, end; 		// for Topological Sort
int f[MAX][4];
int *to[MAX], hi[MAX], cnt[MAX];
char e[MAX][4]; int m[50002];
char mk[MAX];
int ans[MAX];
char ord[4][4][4][4], idx[24][4];

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

void genperm()
{
	int k, t, th;
	char c[4], *pc, *q;
	char p[4] = {1,1,1,1};

	q = p, pc = c;
	for (k = 1; k <= K; ) *q++ = *pc++ = k++;
	k = 1, pc = c, th = 0;
	do {
		t = *(p + k);
		*(p + k) = *(q = p + ((k & 1)? *pc: 0));
		*q = t;

ord[p[0]-1][p[1]-1][p[2]-1][p[3]-1] = th;
idx[th][0] = p[0]-1, idx[th][1] = p[1]-1;
idx[th][2] = p[2]-1, idx[th][3] = p[3]-1;

		th++;
		k = 1, pc = c;
		while(*pc == 0)	*pc++ = k++;
		(*pc)--;
	} while (k < K);
}

int main() 
{
	int D, R, i, j, k, a, b;
	int done, qid;
	ll k1; int k2;

	N = in(), K = in();
	genperm();
	for (i = 0; i < N; i++) for (j = 0; j < K; j++) f[i][j] = in();

	D = in(); if (D > 0) {
		int *memo = malloc(D << 3), j = 0;
		for (i = 0; i < D; i++) {
			memo[j++] = a = in()-1, memo[j++] = b = in()-1;
			hi[a]++;
		}
		for (i = 0; i < N; i++) if (hi[i] > 0) to[i] = malloc(hi[i] << 2);
		memset(hi, 0, N << 2);
		j = 0; while (D--) {
			a = memo[j++], b = memo[j++];
			to[a][hi[a]++] = b;
			cnt[b]++;
		}
		free(memo);
	}

	for (j = 0; j < K; j++) e[0][j] = in()-1;
	R = in();
	for (i = 1; i <= R; i++) {
		m[i-1] = in();
		for (j = 0; j < K; j++) e[i][j] = in()-1;
	}
	m[i-1] = N;

	done = 0, k = 0, top = end = 0, qid = ord[e[0][0]][e[0][1]][e[0][2]][e[0][3]];
    for (a = 0; a < N; a++) if (!cnt[a]) q[end++] = a;

	while (done < N) {
		while (top < end) {
			a = q[top++];
			k2 = 0;
			for (i = 0; i < sz[K]; i++) {
			                k1 =              f[a][idx[i][0]];
				if (K >= 2) k1 = (k1 << 17) | f[a][idx[i][1]];
				if (K >= 3) k1 = (k1 << 17) | f[a][idx[i][2]];
				if (K == 4) k2 =              f[a][idx[i][3]];
				enq(i, a, k1, k2);
			}
		}

		if (done >= m[k]) k++, qid = ord[e[k][0]][e[k][1]][e[k][2]][e[k][3]];
		do a = deq(qid); while (mk[a]);
		mk[a] = 1;
		ans[done++] = a;

		for (i = 0; i < hi[a]; i++) {
			b = to[a][i]; if (--cnt[b] == 0) q[end++] = b;
		}
    }
	for (i = 0; i < N; i++) printf("%d\n", ans[i]+1);
	return 0;
}
