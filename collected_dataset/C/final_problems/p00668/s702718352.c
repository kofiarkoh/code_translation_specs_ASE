// AOJ 1083 The Incubator
// 2019.3.31 bal4u

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
	while (isspace(c)) c = gc();
	do n = 10 * n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

void out(int n)  // 非負整数の表示
{
	int i;
	char b[30];

	if (!n) pc('0');
	else {
		i = 0; while (n) b[i++] = n % 10 + '0', n /= 10;
		while (i--) pc(b[i]);
	}
	pc('\n');
}

void outs(char *s) { while (*s) pc(*s++); }    // 文字列の表示


//// 数値のハッシュ関数
#define HASHSIZ 1500007     // 最大40万のデータに対して
typedef struct { int id, x; } HASH;
HASH hash[HASHSIZ + 5], *hashend = hash + HASHSIZ;

int hash_lookup(int x)
{
	HASH *p = hash + x % HASHSIZ;
	while (p->id) {
		if (p->x == x) return p->id;
		if (++p == hashend) p = hash;
	}
	return -1;
}

void hash_insert(int id, int x)
{
	HASH *p = hash + x % HASHSIZ;
	while (p->id) {
		if (++p == hashend) p = hash;
	}
	p->id = id, p->x = x;
}


//// fenwick_tree_set
//// k番目(0-index) に小さいデータがわかる
#define MAXB 400010
int bit_n, bit_cnt;
int bit_data[MAXB+5];

void init_bit(int sz)
{
	bit_n = sz;
	memset(bit_data, 0, sizeof(bit_data));
}

int find(int p)
{
	int ans = 0;
	while (p > 0) ans += bit_data[p], p -= p & -p;
	return ans;
}

void add(int p, int x)
{
	p++;
	while (p <= bit_n) bit_data[p] += x, p += p & -p;
}

#if 0
int size() { return bit_cnt; }
int count(int val) {            // valという数値の個数
//	assert(0 <= val && val < bit_n);
	return find(val + 1) - find(val);
}
#endif

void insert(int val) {
	//	assert(0 <= val && val < bit_n);
	add(val, 1);
	bit_cnt++;
}

void erase(int val)
{
	//	assert(0 <= val && val < bit_n);
	//	assert(0 < count(val));
	add(val, -1);
	bit_cnt--;
}

// gcc 内蔵 __builtin_clz(n): the number of leading 0-bits in n と同様か
int _clzsi2(unsigned x)
{
	unsigned y;
	int n = 32;
	y = x >> 16; if (y != 0) { n = n - 16; x = y; }
	y = x >> 8;  if (y != 0) { n = n - 8;  x = y; }
	y = x >> 4;  if (y != 0) { n = n - 4;  x = y; }
	y = x >> 2;  if (y != 0) { n = n - 2;  x = y; }
	y = x >> 1;  if (y != 0) return n - 2;
	return n - x;
}

int kth_element(int k) {        // 0番目から 0-index
//	assert(0 <= k && k < bit_cnt);
	int p = 1 << (32 - _clzsi2(bit_n)); 
	int ans = 0;

	while (p >>= 1) {
		if (ans + p <= bit_n && bit_data[ans + p] <= k) {
			k -= bit_data[ans + p];
			ans += p;
		}
	}
	return ans;
};


//// 本問題関連
#define MAX 400010
int xtbl[MAX];         // 時系列に沿って1から付けるIDと個体番号x との対応表
char f[MAX];           // そのIDの個体がキューに残っているかどうか
int q[MAX]; int top, end, len;   // キュー
int id;                // 個体につけるID（１から）

int main()
{
	int c, x, lim, Q;

	while (Q = in()) {
		lim = in();

		// 初期化
		id = 1;        // IDは1から。ハッシュテーブルでの初期化ゼロと区別するため
		memset(hash, 0, sizeof(hash));    // ハッシュテーブルの初期化
		init_bit(MAXB);        // bit木の初期化
		top = end = len = 0;   // キューの初期化

		// 各種クエリを時系列順に処理
		while (Q--) {
			c = in();       // query id
			x = in();
			if (c == 0) {      // 個体に番号をつける
				hash_insert(id, x); // 個体番号をハッシュテーブルに登録し、代わりにIDをキューに追加
				xtbl[id] = x;       // IDからXを知るためのテーブル
				insert(id);         // IDをBIT木に登録
				q[end++] = id;
				len++;
				f[id++] = 1;
			}
			else if (c == 2) { // x番目(1-index)の個体の番号表示
				x = kth_element(x - 1); // BIT木から(x-1)番目のIDを取得
				out(xtbl[x]);    // IDからハッシュテーブルに登録した個体番号を得て表示
			}
			else {             // queryの1,3をまとめて処理
				if (c == 1)    // x番目(1-index)の個体を削除
					x = kth_element(x - 1); // BIT木から(x-1)番目のIDを取得
				else           // 個体番号xを削除
					x = hash_lookup(x);     // IDからハッシュテーブルに登録した個体番号を得て削除
				erase(x);
				f[x] = 0;
				len--;
			}
			while (len > lim) {   // キューの状態を確認。必要であれば、先頭から削除
				x = q[top++];
				if (f[x]) erase(x), f[x] = 0, len--;
			}
		}
		outs("end\n");
	}
	return 0;
}
