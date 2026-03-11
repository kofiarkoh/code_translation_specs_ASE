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

int in()   // 正整数の入力
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

void out(int n)
{
	int i;
	char ob[20];

	i = 0; while (n) ob[i++] = n % 10 + '0', n /= 10;
	while (i--) pc(ob[i]);
	pc('\n');
}

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
char buf[2000002]; int sz;
int len[100005]; char *S[100005];
int sta, ans;

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

#if 0
int myStrcmp(int a, int b)
{
/*
	if (v[a] != v[b]) {
		if (v[a] < v[b]) return -1;
		return 1;
	}
*/
	return strcmp(S[a], S[b]);
}
#else
#define myStrcmp(a,b)  strcmp(S[a],S[b])
#endif

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

void insert(ptr *pp, int id)
{
	ptr p = *pp;
	int k;

	if (p == NULL) {
		*pp = p = tbl + tsize++;
		p->id = id, p->bal = 0;
		p->left = p->right = NULL;
	}
	else {
		k = myStrcmp(id, p->id);
		if (k > 0) {
			insert(&p->right, id);
			p->bal++;
			if (p->bal == 1) leftrotate(pp);
			else if (p->bal == 2) {
				if (p->right->bal == -1) rightrotate(&p->right);
				leftrotate(pp);
			}
		}
		else {  // if (k < 0) {
			insert(&p->left, id);
			p->bal--;
			if (p->bal == -2) {
				if (p->left->bal == 1) leftrotate(&p->left);
				rightrotate(pp);
			}
		}
	}
}

void delete(ptr *pp, int id)
{
	ptr p = *pp;
	int k;

	if (p == NULL) return;
	k = myStrcmp(id, p->id);
	if (k < 0) {
		delete(&p->left, id);
		p->bal++;
		if (p->bal == 2) {
			if (p->right->bal == -1) rightrotate(&p->right);
			leftrotate(pp);
		}
	}
	else if (k > 0) {
		delete(&p->right, id);
		p->bal--;
		if (p->bal == -2) {
			if (p->left->bal == 1) leftrotate(&p->left);
			rightrotate(pp);
		}
	}
	else {   // if (k == 0 {
		if (p->right == NULL) {
			*pp = p->left;
			//			free(p);
		}
		else if (p->left == NULL) {
			*pp = p->right;
			//			free(p);
		}
		else {
			ptr *qq = &p->left;
			while ((*qq)->right != NULL) qq = &(*qq)->right;
			p->id = (*qq)->id;
			(*qq)->id = id;
			delete(&p->left, id);
			p->bal++;
			if (p->bal == 2) {
				if (p->right->bal == -1) rightrotate(&p->right);
				leftrotate(pp);
			}
		}
	}
}

void search(ptr *pp, int id)
{
	ptr p = *pp;
	int k;

	if (sta > 1) return;
	if (p == NULL) {
		if (sta > 0) return;
		ans = -1, sta = 1;
		return;
	}
/*
	printf("search sta=%d, ans=%d: id(%d), s(%d)=%s <-> id(%d), s(%d)=%s\n",
		sta, ans, id, len[id], S[id], p->id, len[p->id], S[p->id]);
*/
	k = strncmp(S[id], S[p->id], len[id]);
	if (k == 0) {
		sta = 1, ans = p->id;
		if (len[p->id] == len[id]) { sta = 2; return; }
		search(&p->left, id);
	}
	else if (k < 0) search(&p->left, id);
	else search(&p->right, id);
}

int main()
{
	int i, Q, k, id;

	N = in();
	for (i = 1; i <= N; i++) {
		len[i] = ins(S[i] = buf + sz);
		sz += len[i] + 1;
	}
/*
	for (i = 1; i <= N; i++) {
		printf("[%d] len=%d, %s\n", i, len[i], S[i]);
	}
*/

	Q = in();
	while (Q--) {
		k = in(), id = in();
		if (k == 1) insert(&root, id);
		else if (k == 2) delete(&root, id);
		else {
			sta = 0;
			search(&root, id);
			if (ans < 0) puts("-1");
			else out(ans);
		}
/*
		printf("----Dump\n");
		dump(&root, 0);
		printf("--------\n\n");
*/
	}
	return 0;
}

