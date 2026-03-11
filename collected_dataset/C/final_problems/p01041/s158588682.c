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

#define TMAX 500000

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

if (p == NULL) return;
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

if (p == NULL) return;
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
//		printf("tsize=%d\n", tsize);
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
	ptr p = *pp;
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
		if (p->id != id) {
			printf("Panic!!! delte: id=%d, len=%d, %s\n", id, len[id], S[id]);
			exit(1);
		}
		if (p->right == NULL) {
			*pp = p->left;
			//			free(p);
			return 1;
		}
		else if (p->left == NULL) {
			*pp = p->right;
			//			free(p);
			return 1;
		}
		else {
			ptr *qq = &p->left;
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
	}
	return ret;
}

void search(ptr *pp, int id)
{
	ptr p = *pp;
	int k;

	if (p == NULL) return;
	/*
		printf("search sta=%d, ans=%d: id(%d), s(%d)=%s <-> id(%d), s(%d)=%s\n",
			sta, ans, id, len[id], S[id], p->id, len[p->id], S[p->id]);
	*/
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
//	printf("N=%d, sz=%d\n", N, sz);
	Q = in();
	while (Q--) {
		k = gc() & 3, gc(), id = in();
		if (k == 1) insert(&root, id);
		else if (k == 2) delete(&root, id);
		else {
			ans = -1;
			search(&root, id);
			if (ans < 0) puts("-1");
			else out(ans);
		}
	}
/*
	printf("----Dump\n");
	dump(&root, 0);
	printf("--------\n\n");
	printf("tsize=%d\n", tsize);
*/
	return 0;
}

