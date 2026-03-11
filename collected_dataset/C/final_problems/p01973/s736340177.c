// AOJ 2873 Censored String
// 2019.3.23 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

//// 高速入力処理
#if 1
#define gc() getchar_unlocked()
#else
#define gc() getchar()
#endif
int in()  // 非負整数の入力
{
	int n = 0, c = gc();
	do n = 10 * n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

int ins(char *s)  // 文字列の入力　スペース以下の文字で入力終了
{
	char *p = s;
	do *s = gc();
	while (*s++ > ' ');
	*--s = 0;
	return s - p;
}

#define MARK (1<<30)
#define MASK (MARK-1)
#define MAX  100005
#define TMAX 500005
typedef struct { signed char ch; int bro, son; } T;  // 文字、兄弟、子供
T t[TMAX + 5]; int sz = 2;
int tbl[27][27][27][27];
char str[MAX], *S;
char buf[MAX];

void setup(int w, char *s)
{
	int i, node, f[4];

	f[1] = f[2] = f[3] = 0;
	for (i = 0; i < 4 && i < w; i++) f[i] = s[i] - 'a' + 1;
	if (w <= 4) {
		tbl[f[0]][f[1]][f[2]][f[3]] |= MARK;
		return;
	}
	if (((node = tbl[f[0]][f[1]][f[2]][f[3]]) & MASK) == 0) {
		tbl[f[0]][f[1]][f[2]][f[3]] |= sz;
		node = sz++;
	}
	node &= MASK;
	s += 4; while (*s) {
		int c = t[node].ch;
		if (c < 0) return;
		if (c == *s) {
			if (t[node].son == 0) t[node].son = sz, node = sz++;
			else node = t[node].son;
			s++;
		}
		else if (c > 0) {
			if (t[node].bro == 0) t[node].bro = sz, node = sz++;
			else node = t[node].bro;
		}
		else {
			t[node].ch = *s++, t[node].son = sz, node = sz++;
		}
	}
	t[node].ch = -1;
}

int resolv()
{
	int i, node, f[4];

	f[1] = f[2] = f[3] = 0;
	for (i = 0; i < 4 && *S; i++) {
		f[i] = *S++ - 'a' + 1;
		if (tbl[f[0]][f[1]][f[2]][f[3]] & MARK) return 1;
	}
	node = tbl[f[0]][f[1]][f[2]][f[3]] & MASK;
	while (*S) {
		int c = t[node].ch;
		if (c < 0) return 1;
		if (c == *S) node = t[node].son, S++;
		else if (c > 0) node = t[node].bro;
		else break;
	}
	return 0;
}

int main()
{
	int i, N, ans;
	char *p, *min, *end;

	end = str + ins(str) + 1;
	N = in(); for (i = 0; i < N; i++) {
		int w = ins(buf);
		setup(w, buf);
	}

	ans = 0, S = str, min = end;
	while (1) {
		if (S >= min) ans++, min = end;
		if (*S == 0) break;
		p = S;
		if (resolv()) {	if (S < min) min = S; }
		S = p + 1;  
	}
	printf("%d\n", ans);
	return 0;
}
