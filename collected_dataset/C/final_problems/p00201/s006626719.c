// Aizu Vol-2 0201: Wrought Gold Master
// 2017.8.17 bal4u@uu
// ??¢?????????????????????????????\????????????????????£?????????

#include <stdio.h>
#include <string.h>

#define HSIZ 499
typedef struct { char *w; int k; } HASH;
HASH hash[HSIZ + 5];

int lookup(char *w)
{
	int i, l;
	HASH *tp;
	i = (101 * *w + 103 * *(w + (l = strlen(w)) - 1) + 107 * l) % HSIZ;
	tp = hash + i;
	if (tp->w != NULL) {
		int inc = *w + (*(w + 1) << 1);
		while (1) {
			if (!strcmp(tp->w, w)) return tp->k;
			i += inc; if (i >= HSIZ) i -= HSIZ;
			tp = hash + i;
			if (tp->w == NULL) break;
		}
	}
	return -1;
}

int insert(char *w, int k)
{
	int i, l;
	HASH *tp;
	i = (101 * *w + 103 * *(w + (l = strlen(w)) - 1) + 107 * l) % HSIZ;
	tp = hash + i;
	if (tp->w != NULL) {
		int inc = *w + (*(w + 1) << 1);
		while (1) {
			if (!strcmp(tp->w, w)) return -1;
			i += inc; if (i >= HSIZ) i -= HSIZ;
			tp = hash + i;
			if (tp->w == NULL) break;
		}
	}
	tp->w = w, tp->k = k;
	return 0;
}

typedef struct { char a[102]; int m, k, i[102]; } MONO;
MONO mono[205]; int no;

int combi(int a)
{
	int i, k, c = -1;
	k = mono[a].k;
	if (k > 0) for (c = i = 0; i < k; i++) c += combi(mono[a].i[i]);
	if (c < 0 || (mono[a].m >= 0 && mono[a].m < c)) c = mono[a].m;
	return c;
}

int main()
{
	int i, j, k, N, M;
	char b[102];

	while (scanf("%d", &N) == 1 && N) {
		memset(hash, 0, sizeof(hash)), no = 0;
		while (N-- > 0) {
			scanf("%s%d", mono[no].a, &(mono[no].m));
			mono[no].k = 0; insert(mono[no].a, no);
			no++;
		}
		scanf("%d", &M);
		while (M--) {
			scanf("%s%d", mono[no].a, &k);
			if ((i = lookup(mono[no].a)) < 0) insert(mono[no].a, no), mono[no].m = -1, i = no++;
			mono[i].k = k;
			for (j = 0; j < mono[i].k; j++) scanf("%s", b), mono[i].i[j] = lookup(b);
		}
		scanf("%s", b);
		printf("%d\n", combi(lookup(b)));
	}
	return 0;
}