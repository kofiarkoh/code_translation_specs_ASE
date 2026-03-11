// AOJ 2644 Longest Match
// 2018.5.13 bal4u

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
#define MAX_L 200005

char text[MAX_L]; int len;
char pa[200005], pb[200005]; int wa, wb;
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

// 接尾辞配列を用いた文字列検索　len = strlen(s), plen = strlen(t)
int lower_bound(int wt, char *t)
{
	int l = 0, r = len, m;
	while (l < r) {
		m = (l+r)>>1;
		if (memcmp(text+sa[m], t, wt) < 0) l = m+1; else r = m;
	}
	return l;
}

int upper_bound(int wt, char *t)
{
	int l = 0, r = len, m;
	while (l < r) {
		m = (l+r)>>1;
		if (memcmp(text+sa[m], t, wt) <= 0) l = m+1; else r = m;
	}
	return l - (memcmp(text+sa[l], t, wt) != 0);
}

/* 空間木 */
// セグメント木の初期化：配列の確保と初期値による初期化
int minseg[1<<19];
int maxseg[1<<19];
int sz;

void segtree(int n, int init_val)
{
	sz = 2; while (sz < n) sz <<= 1;
	n = sz << 1;
	memset(minseg,  init_val, n << 2);
//	memset(maxseg, -init_val, n << 2);
}

#if 0
// k番目（0-index）の値を v に変更
void update(int k, int v)
{
	int p, pp;

	k += sz - 1;
	minseg[k] = v, maxseg[k] = v;
	while (k > 0) {
		if (!(k & 1)) k--;
		pp = p = k, k >>= 1;
		v = minseg[p++]; if (v > minseg[p]) v = minseg[p];
		minseg[k] = v;

		p = pp;
		v = maxseg[p++]; if (v < maxseg[p]) v = maxseg[p];
		maxseg[k] = v;
	}
}
#endif

// RMQのインデックス部分を一括して更新
void update()
{
	int i, k, n, p;

	n = sz;	while (n > 0) {
		for (p = n-1, i = 0; i < n; i+=2, p+=2) {
			k = p >> 1;

			if (minseg[p] <= minseg[p+1])
				 minseg[k] = minseg[p  ], minseg[k] = minseg[p  ];
			else minseg[k] = minseg[p+1], minseg[k] = minseg[p+1];

			if (maxseg[p] >= maxseg[p+1])
				 maxseg[k] = maxseg[p  ], maxseg[k] = maxseg[p  ];
			else maxseg[k] = maxseg[p+1], maxseg[k] = maxseg[p+1];
		}
		n >>= 1;
	}
}

int query2min(int a, int b, int k, int l, int r) 
{
	int m, lmin, rmin;

	if (r <= a || b <= l) return INF;
    if (a <= l && r <= b) return minseg[k];
	k <<= 1, m = (l+r) >> 1;
    lmin = query2min(a, b, k+1, l, m);
    rmin = query2min(a, b, k+2, m, r);
	if (lmin > rmin) lmin = rmin;
    return lmin; 
}

int query2max(int a, int b, int k, int l, int r) 
{
	int m, lmax, rmax;

	if (r <= a || b <= l) return -1;
    if (a <= l && r <= b) return maxseg[k];
	k <<= 1, m = (l+r) >> 1;
    lmax = query2max(a, b, k+1, l, m);
    rmax = query2max(a, b, k+2, m, r);
	if (lmax < rmax) lmax = rmax;
    return lmax; 
}

int main()
{
	int i, m;
	int la, lb, ra, rb, l, r;
	int xx, xa, xb;

	len = str(&xx, text);
	if (xx < 0) {
		construct_sa(text, sa);
		segtree(len+1, INF);
		m = sz - 1;
		for (i = 0; i <= len; i++) minseg[m] = maxseg[m] = sa[i], m++;
		update();	// 一括して更新
	}

	m = in();
	while (m--) {
		wa = str(&xa, pa);
		wb = str(&xb, pb);
		if (xx > 0) {
			if (xa < 0 || xb < 0 ||
				xa != xx || xb != xx || wa > len || wb > len) goto done;
			out(len);
			continue;
		}

		if (wa > len || wb > len) goto done;
		if (wa + wb == len) {
			if (memcmp(text, pa, wa)) goto done;
			if (memcmp(text+wa, pb, wb)) goto done;
			out(len);
			continue;
		}
		ra = upper_bound(wa, pa);
		if (ra > len) goto done;
		rb = upper_bound(wb, pb);
		if (rb > len) goto done;

		la = lower_bound(wa, pa);
		if (la > ra) goto done;
		lb = lower_bound(wb, pb);
		if (lb > rb) goto done;

		l = query2min(la, ra+1, 0, 0, sz);
		r = query2max(lb, rb+1, 0, 0, sz);
		if (l <= r && l+wa <= r+wb) out(r-l+wb);
		else {
done:		out(0);
		}
	}
	return 0;
}
