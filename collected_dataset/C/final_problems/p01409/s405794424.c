// AOJ 2292 Common Palindromes
// 2019.8.16 bal4u

#include <stdio.h>
#include <string.h>

typedef long long ll;

#if 1
#define gc() getchar_unlocked()
#else
#define gc() getchar()
#endif

int ins(char *s)  // 文字列の入力　スペース以下の文字で入力終了
{
	char *p = s;
	do *s = gc();
	while (*s++ > ' ');
	*--s = 0;
	return s - p;
}

#define MAX 100010
//// 回文木の構築
typedef struct { int par, son[30], suffix_link, len, cnt; } NODE;
NODE node[MAX]; int sz = 2;
int last;
char S[MAX]; int W;  // 文字列

void init_palindromic_tree() { // 回文木の初期化
	int i;
	
	for (i = 0; i < MAX; i++) {
		node[i].par = -1;
		memset(node[i].son, -1, sizeof(node[i].son));
	}
	node[0].suffix_link = -1, node[0].len = -1;
	node[1].par = 0;
}

void add_palindromic_tree(int idx) { // 文字を回文木に追加。文字位置 S[idx]
	char c = S[idx]; int a = S[idx] - '>';
	int t = last;

	while (1) {
		if (idx-1-node[t].len >= 0 && S[idx-1-node[t].len] == c) break;
		t = node[t].suffix_link;
	}

	if (node[t].son[a] > 0) {
		last = node[t].son[a], node[last].cnt++;
		return;
	}

	last = sz++;
	node[t].son[a] = last;

	node[last].len = node[t].len + 2, node[last].par = t, node[last].cnt = 1;

	if (node[last].len == 1) node[last].suffix_link = 1;
	else while (1) {
		t = node[t].suffix_link;
		if (idx-1-node[t].len >= 0 && S[idx-1-node[t].len] == c) {
			node[last].suffix_link = node[t].son[a];
			break;
		}
	}
}

char *ss; int ws;
char *tt; int wt; 
int f[2][MAX];

int main()
{
	int i, t; ll ans;
	
	ws = ins(ss = S);
	S[ws] = '>', S[ws+1] = '@';
	wt = ins(tt = ss + ws + 2);
	W = ws + wt + 2;
	
	init_palindromic_tree();
	for (i = 0; i < ws; i++) {
		add_palindromic_tree(i);
		f[0][last]++;
	}
	add_palindromic_tree(i++), add_palindromic_tree(i++);
	for ( ; i < W; i++) {
		add_palindromic_tree(i);
		f[1][last]++;
	}

	ans = 0;
	for (i = sz-1; i > 1; i--) {
		ans += (ll)f[0][i] * f[1][i];
		f[0][node[i].suffix_link] += f[0][i];
		f[1][node[i].suffix_link] += f[1][i];
	}
	printf("%lld\n",ans);
    return 0;
}
