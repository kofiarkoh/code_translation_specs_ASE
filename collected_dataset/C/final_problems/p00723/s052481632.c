// Aizu 1142: Organize Your Train part II
// 2017.9.15 bal4u@uu

#include <stdio.h>
#include <string.h>

#define HSIZ 1201
typedef struct { char *w; int k; } HASH;
HASH hash[HSIZ + 5];
char s[600][75];
int ans;

int insert(char *w, int l, int k)
{
	int i;
	HASH *tp;
	i = (101 * *w + 103 * *(w + l - 1) + 107 * l) % HSIZ;
	tp = hash + i;
	if (tp->w != NULL) {
		int inc = *w + (*(w + 1) << 1);
		while (1) {
			if (!strcmp(tp->w, w)) return 0;
			i += inc; if (i >= HSIZ) i -= HSIZ;
			tp = hash + i;
			if (tp->w == NULL) break;
		}
	}
	tp->w = w, tp->k = k;
	return 1;
}

void combi(int k, char *s1, int w1, char *s2, int w2)
{
	char *d = s[k];
	memcpy(d, s1, w1), memcpy(d+w1, s2, w2), *(d+w1+w2) = 0;
	ans += insert(s[k], w1+w2, 0);
}

void reverse(char *r, char *s, int w)
{
	*(r+w) = 0,	r += w; while (w-- > 0) *--r = *s++; 
}

int main()
{
	int n, i, w, w1, w2;
	char *p, s1[75], s2[72], r1[75], r2[75];

	scanf("%d", &n);
	while (n-- > 0) {
		memset(hash, 0, sizeof(hash));
		scanf("%s", p = s[0]); w = strlen(p);
		ans = insert(p, w, 0);
		for (i = 1, w1 = 1; w1 < w; w1++) {
			w2 = w - w1;
			memcpy(s1, p, w1), s1[w1] = 0;
			memcpy(s2, p+w1, w2), s2[w2] = 0;
			reverse(r1, s1, w1), reverse(r2, s2, w2);

			combi(i++, s1, w1, r2, w2);
			combi(i++, r1, w1, s2, w2);
			combi(i++, r1, w1, r2, w2);
			combi(i++, s2, w2, s1, w1);
			combi(i++, s2, w2, r1, w1);
			combi(i++, r2, w2, s1, w1);
			combi(i++, r2, w2, r1, w1);
		}
		printf("%d\n", ans);
	}
	return 0;
}