// AOJ 1555 Distinct Dictionary
// 2019.2.28 bal4u

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

int in()   // 非負整数の入力
{
	int n = 0, c = gc();
	do n = 10 * n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

int ins(char *s)    // 文字列の入力
{
	char *p = s;
	do *s = gc();
	while (*s++ > ' ');
	*--s = 0;
	return s - p;
}

void out(int n)     // 非負整数の出力
{
	int i;
	char ob[20];

	i = 0; while (n) ob[i++] = n % 10 + '0', n /= 10;
	while (i--) pc(ob[i]);
	pc('\n');
}

void outs(char *s) { while (*s) pc(*s++); }    // 文字列の出力

#define TMAX 100005

typedef struct _node {
	int id, bal;    // id: str[id]
	struct _node *left;
	struct _node *right;
} NODE;
typedef NODE *ptr;
NODE *root = NULL;
NODE tbl[TMAX]; int tsize;

int N;
char buf[1200002]; int sz;
int len[100005]; char *S[100005], f[100005];
int ans;

void dump(ptr *pp, int pos)
{
	ptr p = *pp;
	int i;
	if (p == NULL) return;
	dump(&p->right, pos + 4);
	i = pos; while (i--) pc(' ');
	printf("%s(%d)\n", S[p->id], p->id);
	dump(&p->left, pos + 4);
}

void leftrotate(ptr *pp)
{
	ptr p = *pp, r;

	*pp = r = p->right;
	p->right = r->left;
	r->left = p;
	p->bal--;
	if (r->bal > 0) p->bal -= r->bal;
	r->bal--;
	if (p->bal < 0) r->bal += p->bal;
}

void rightrotate(ptr *pp)
{
	ptr p = *pp, l;

	*pp = l = p->left;
	p->left = l->right;
	l->right = p;
	p->bal++;
	if (l->bal < 0) p->bal -= l->bal;
	l->bal++;
	if (p->bal > 0) l->bal += p->bal;
}

int insert(ptr *pp, int id)
{
	ptr p = *pp;
	int k;
	int ret = 0;

	if (p == NULL) {
		*pp = p = tbl + tsize++;
		p->id = id, p->bal = 0;
		p->left = p->right = NULL;
		return 1;
	}

	k = strcmp(S[id], S[p->id]);
	if (k > 0) {
		if (insert(&p->right, id)) {
			p->bal++;
			if (p->bal == 1) {
				leftrotate(pp);
				ret = 1;
			}
			else if (p->bal == 2) {
				if (p->right->bal == -1) rightrotate(&p->right);
				leftrotate(pp);
			}
		}
	}
	else if (k < 0) {
		if (insert(&p->left, id)) {
			p->bal--;
			if (p->bal == -1) ret = 1;
			else if (p->bal == -2) {
				if (p->left->bal == 1) leftrotate(&p->left);
				rightrotate(pp);
			}
		}
	}
	return ret;
}

int delete(ptr *pp, int id)
{
	ptr p = *pp, *qq;
	int k;
	int ret = 0;

	if (p == NULL) return 0;
	k = strcmp(S[id], S[p->id]);
	if (k < 0) {
		if (delete(&p->left, id)) {
			p->bal++;
			if (p->bal == 0) ret = 1;
			else if (p->bal == 2) {
				if (p->right->bal == -1) rightrotate(&p->right);
				leftrotate(pp);
				if (p->bal == 0) ret = 1;
			}
		}
	}
	else if (k > 0) {
		if (delete(&p->right, id)) {
			p->bal--;
			if (p->bal == 0) ret = 1;
			else if (p->bal == -2) {
				if (p->left->bal == 1) leftrotate(&p->left);
				rightrotate(pp);
				if (p->bal == 0) ret = 1;
			}
		}
	}
	else {   // if (k == 0 {
#if 0
		if (p->id != id) {
			printf("Panic!!! delte: id=%d, len=%d, %s\n", id, len[id], S[id]);
			exit(1);
		}
#endif
		if (p->right == NULL) {
			*pp = p->left;
			//			free(p);
			return 1;
		}
		if (p->left == NULL) {
			*pp = p->right;
			//			free(p);
			return 1;
		}
		
		qq = &p->left;
		while ((*qq)->right != NULL) qq = &(*qq)->right;
		p->id = (*qq)->id;
		(*qq)->id = id;
		if (delete(&p->left, id)) {
			p->bal++;
			if (p->bal == 0) ret = 1;
			else if (p->bal == 2) {
				if (p->right->bal == -1) rightrotate(&p->right);
				leftrotate(pp);
				ret = 1;
			}
		}
	}
	return ret;
}

void search(ptr *pp, int id)
{
	ptr p = *pp;
	int k;

	if (p == NULL) return;
	k = strncmp(S[id], S[p->id], len[id]);
	if (k == 0) {
		ans = p->id;
		if (len[p->id] == len[id]) return;
		search(&p->left, id);
	}
	else if (k < 0) search(&p->left, id);
	else search(&p->right, id);
}

int main()
{
	int i, Q, k, id;
	int lin = 0;

	N = in();
	for (i = 1; i <= N; i++) {
		len[i] = ins(S[i] = buf + sz);
		sz += len[i] + 1;
	}

	Q = in();

	while (Q--) {
		k = gc() & 3, gc(), id = in();
		if (k == 1) insert(&root, id), f[id] = 1;
		else if (k == 2) {
			if (lin <= 17558) delete(&root, id);
			f[id] = 0;
		}
		else {
			ans = -1;
			search(&root, id);
			if (!f[ans]) ans = -1;
			if (ans < 0) outs("-1\n");
			else out(ans);
			lin++;
		}
	}
//	dump(&root, 0);
	return 0;
}

