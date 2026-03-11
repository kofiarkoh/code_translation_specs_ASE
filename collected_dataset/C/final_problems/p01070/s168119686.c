// AOJ 1585 String in String
// 2018.5.20 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#if 1
#define gc() getchar_unlocked()
#define pc(c) putchar_unlocked(c)
#else
#define gc() getchar()
#define pc(c) putchar(c)
#endif

int in()
{
	int n = 0, c = gc();
	do n = 10*n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

void out(int n)
{
	int i;
	char ob[20];

	if (!n) pc('0');
	else {
		i = 0; while (n) ob[i++] = n%10 + '0', n/=10;
		while (i--) pc(ob[i]);
	}
	pc('\n');
}

int str(int *f, char *s)
{
	int x;
	char *p = s, c;

	*p++ = x = gc();
	while ((c = gc()) > ' ') {
		*p++ = c;
		if (c != x) x = -1;
	}
	*p = 0;
	*f = x;
	return p-s;
}

#define INF   0x10101010
#define MAX_L 100005

char text[MAX_L]; int len;
char pat[MAX_L]; int wp;
int  sa[MAX_L], rank[MAX_L<<1];

/* 接尾辞配列 */
// rank[i]～[i+k] と rank[j]～[j+k] を比較
int k; // comstruct_sa()内で設定
int compare_sa(int i, int j)
{
	if (rank[i] != rank[j]) return rank[i] < rank[j];
	return rank[i+k] < rank[j+k];
}

// for qsort()
int cmp(const void *a, const void *b)
{
	int i = *(const int *)a, j = *(const int *)b;
	
	if (rank[i] != rank[j]) return rank[i] - rank[j];
	return rank[i+k] - rank[j+k];
}

// 文字列Sの接尾辞配列saを構築。   len:　strlen(s)
int tmp[MAX_L];
void construct_sa(char *s, int *sa)
{
	int i;
	
	memset(rank, -1, sizeof(rank));
	for(i = 0; i <= len; i++) {
		sa[i] = i;
		if (i < len) rank[i] = s[i];
	}
	
	for(k = 1; k <= len; k *= 2) {
		qsort(sa, len+1, sizeof(int), cmp);
		
		tmp[sa[0]] = 0;
		for(i = 1; i <= len; i++) {
			tmp[sa[i]] = tmp[sa[i-1]] + compare_sa(sa[i-1], sa[i]);
		}
		for(i = 0; i <= len; i++) rank[i] = tmp[i];
	}
}

int main()
{
	int Q, l, r, lo, hi, mi, ans;
	int xx, xp;

	len = str(&xx, text);
	if (xx < 0) construct_sa(text, sa);

	Q = in();
	while (Q--) {
		l = in(), r = in()+1, wp = str(&xp, pat);

		if (wp > r-l) {	pc('0'), pc('\n'); continue; }
		if (wp == r-l) {
			pc ('0' + (memcmp(text+l, pat, wp) == 0)), pc('\n');
			continue;
		}
	
		if (xx > 0) {
			if (xp < 0 || xp != xx || wp > len) { pc('0'), pc('\n'); continue; }
			out(r-l+1-wp);
			continue;
		}

		lo = 0, hi = len;
		while (lo < hi) {
			mi = (lo + hi) >> 1;
			if (memcmp(text+sa[mi], pat, wp) < 0) lo = mi+1; else hi = mi;
		}
		
		ans = 0, r -= wp;
		while (lo <= len) {
			if (memcmp(text+sa[lo], pat, wp)) break;
		    if (l <= sa[lo] && sa[lo] <= r) ans++;
			lo++;
		}
		out(ans);
	}
	return 0;
}
