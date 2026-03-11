// AOJ 1515 The Humans Braving the Invaders
// 2019.4.1 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

//// 高速入出力
#if 1
#define gc() getchar_unlocked()
#define pc(c) putchar_unlocked(c)
#else
#define gc() getchar()
#define pc(c) putchar(c)
#endif
int in()  // 非負整数の入力
{
	int n = 0, c = gc();
//	while (isspace(c)) c = gc();
	do n = 10 * n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

void out(int n)  // 整数の表示
{
	int i;
	char b[30];

	if (!n) pc('0');
	else {
//		if (n < 0) n = -n, pc('-');
		i = 0; while (n) b[i++] = n % 10 + '0', n /= 10;
		while (i--) pc(b[i]);
	}
}

void outs(char *s) { while (*s) pc(*s++); }    // 文字列の表示


//// fenwick_tree_set
// 空間加算
#define MAXB 100005
int bit1_data[MAXB + 5];
int bit2_data[MAXB + 5];
int bit_n, bit_cnt;

void bit_init(int sz) {
	bit_n = sz, bit_cnt = 0;
	memset(bit1_data, 0, sizeof(bit1_data));
	memset(bit2_data, 0, sizeof(bit2_data));
}
int bit_query(int p) {   // 1-index
	int sum = 0;
	while (p) sum += bit1_data[p], p -= p & -p;
	return sum;
}
void bit_add(int p, int v) { while (p <= bit_n) bit1_data[p] += v, p += p & -p; }

void range_add(int l, int r, int v) { // 空間[l,r]にv加算
	bit_add(l, v);
	bit_add(r + 1, -v);
}

// k番目IDの取得
void add(int p, int x) { p++; while (p <= bit_n) bit2_data[p] += x, p += p & -p; }
void insert(int val) { add(val, 1), bit_cnt++; }
void erase(int val) { add(val, -1), bit_cnt--; }
int _clzsi2(unsigned x) {
	unsigned y;	int n = 32;
	y = x >> 16; if (y != 0) { n = n - 16; x = y; }
	y = x >> 8;  if (y != 0) { n = n - 8;  x = y; }
	y = x >> 4;  if (y != 0) { n = n - 4;  x = y; }
	y = x >> 2;  if (y != 0) { n = n - 2;  x = y; }
	y = x >> 1;  if (y != 0) return n - 2;
	return n - x;
}
int kth_element(int k) {      // 0-index
	int p = 1 << (32 - _clzsi2(bit_n));
	int ans = 0;
	while (p >>= 1) if (ans + p <= bit_n && bit2_data[ans + p] <= k)
		k -= bit2_data[ans + p], ans += p;
	return ans;
};


//// 本問題関連
#define MAX 100010
char f[MAX];           // そのIDのインベーダーが現存しているかどうか
int top;               // 生存中インベーダー集団の先頭
int id;                // 個体につけるID（１から）
int L;
int cnt;               // 生存しているインベーダーの数

// バイナリサーチ
int bsch_upper(int x)
{
	int m, l, r;

	l = top, r = id + 1;
	while (l < r) {
		m = (l + r) >> 1;
		if (bit_query(m) < x) l = m + 1; else r = m;
	}
	return l;
}

int bsch_lower(int x)
{
	int m, l, r;

	l = top, r = id + 1;
	while (l < r) {
		m = (l + r) >> 1;
		if (bit_query(m) <= x) l = m + 1; else r = m;
	}
	return l-1;
}

int main()
{
	int i, Q, c, k, s, a, b;
	int x, r, d;

	while (Q = in()) {
		L = in();

		// 初期化
		id = 0;                      // IDは1から
		bit_init(MAXB);              // bit木の初期化
		memset(f, 0, sizeof(f));
		top = 1, cnt = 0;

		// 各種クエリを時系列順に処理
		while (Q--) {
			c = in();      // query id
			if (c == 0) {          // インベーダーが出現
				insert(++id);      // IDをk番目ID取得用BIT木に登録
				range_add(id, id, L);
				if (top > id) top = id;
				f[id] = 1;
				cnt++;
			}
			else if (c == 1) {     // インベーダーが距離dだけ移動
				d = -in();
				if (cnt == 0) continue;
				range_add(top, id, d);
				i = bsch_lower(0);
				if (i < top) i = top;
				if (i > id) i = id;
				if (bit_query(i) <= 0) {
					s = 0;
					while (top <= i) {
						if (f[top]) s++, erase(top), f[top] = 0;
						top++;
					}
					if (s > 0) outs("damage "), out(s), pc('\n'), cnt -= s;
				}
			}
			else if (c == 2) {   // k番目のインベーダーを撃墜
				k = in();
				if (k > cnt) outs("miss\n");
				else {
					i = kth_element(k - 1); // BIT木から(x-1)番目のIDを取得
					f[i] = 0, cnt--, erase(i);
					outs("hit\n");
				}
			}
			else if (c == 3) {  // 広範囲にインベーダーを攻撃
				x = in(), r = in();
				if (cnt == 0) { outs("bomb 0\n"); continue; }
				if (x - r <= 0) a = top;
				else a = bsch_upper(x - r);
				if (x + r >= L) b = id;
				else b = bsch_lower(x + r);
				if (a < top) a = top;
				while (a < id) {
					if (bit_query(a) >= x - r) break;
					a++;
				}
				if (b > id) b = id;
				while (a <= b) {
					if (bit_query(b) <= x + r) break;
					b--;
				}
				s = 0;  while (a <= b) {
					if (f[a]) s++, erase(a), f[a] = 0;
					if (top == a) top++;
					a++;
				}
				outs("bomb "), out(s), pc('\n'), cnt -= s;
			}
			else {              // 距離の報告
				k = in();
				if (k > cnt) outs("distance -1\n");
				else {
					i = kth_element(k - 1); // BIT木から(x-1)番目のIDを取得
					outs("distance "), out(bit_query(i)), pc('\n');
				}
			}
		}
		outs("end\n");
	}
	return 0;
}
