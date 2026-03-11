#include <stdio.h>

unsigned xor128() {
	static unsigned x = 123456789, y = 362436069, z = 521288629, w = 88675123;
	unsigned t = x ^ (x << 11);
	x = y; y = z; z = w;
	return w = w ^ (w >> 19) ^ (t ^ (t >> 8));
}

typedef struct node_t {
	int val;
	struct node_t *lch, *rch;
	int size;
	unsigned int pri;
} node;

node* new_node(int val) {
	static int it = 0;
	static node pool[200000];
	pool[it] = (node) { val, 0, 0, 1, xor128() };
	return &pool[it++];
}

int count(node *t) {
	return !t ? 0 : t->size;
}

node *update(node *t) {
	t->size = count(t->lch) + count(t->rch) + 1;
	return t;
}

node *merge(node *l, node *r) {
	if (!l || !r) return !l ? r : l;
	if (l->pri > r->pri) {
		l->rch = merge(l->rch, r);
		return update(l);
	}
	r->lch = merge(l, r->lch);
	return update(r);
}

typedef struct{ node *a, *b; } pnode;

pnode split(node *t, int k) {
	if (!t) return (pnode) { 0, 0 };
	if (k <= count(t->lch)) {
		pnode s = split(t->lch, k);
		t->lch = s.b;
		return (pnode) { s.a, update(t) };
	}
	pnode s = split(t->rch, k - count(t->lch) - 1);
	t->rch = s.a;
	return (pnode) { update(t), s.b };
}

int count_lower(node *t, int v) {
	int res = 0;
	while (t) {
		if (t->val < v) {
			res += count(t->lch) + 1;
			t = t->rch;
		}
		else {
			t = t->lch;
		}
	}
	return res;
}

node *insert(node *t, int val) {
	int k = count_lower(t, val);
	pnode s = split(t, k);
	return merge(merge(s.a, new_node(val)), s.b);
}

node *erase(node *t, int l, int r) {
	pnode s1 = split(t, l), s2 = split(s1.b, r - l);
	return merge(s1.a, s2.b);
}

void print(node *t) {
	if (!t) return;
	print(t->lch);
	printf("%d\n", t->val);
	print(t->rch);
}

int main(void)
{
	int q, com, x, l, r, cnt = 0;
	node *root = 0;
	pnode p1, p2;
	scanf("%d", &q);
	while (q--) {
		scanf("%d%d", &com, &x);
		if (com == 0) {
			root = insert(root, x);
			printf("%d\n", ++cnt);
		}
		else if (com == 1) {
			printf("%d\n", count_lower(root, x + 1) - count_lower(root, x));
		}
		else if (com == 2) {
			l = count_lower(root, x);
			r = count_lower(root, x + 1);
			cnt -= r - l;
			root = erase(root, l, r);
		}
		else {
			scanf("%d", &r);
			l = count_lower(root, x);
			r = count_lower(root, r + 1);
			p1 = split(root, l), p2 = split(p1.b, r - l);
			print(p2.a);
			root = merge(merge(p1.a, p2.a), p2.b);
		}
	}
	return 0;
}

