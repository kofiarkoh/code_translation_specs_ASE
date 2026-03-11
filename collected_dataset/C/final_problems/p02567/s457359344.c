#include <stdio.h>

const int inf = -1;
int leaf[200001];

typedef struct {
	int left, right, max;
} seg_node;

void init_node(seg_node v[], int k, int l, int r)
{
	v[k].left = l;
	v[k].right = r;
	v[k].max = inf;
	if (l < r) {
		init_node(v, k << 1, l, (l + r) / 2);
		init_node(v, (k << 1) ^ 1, (l + r) / 2 + 1, r);
	} else leaf[l] = k;
}

void update_node(seg_node v[], int k, int x)
{
	int i, j = leaf[k];
	v[j].max = x;
	for (i = j >> 1; i > 0; j = i, i >>= 1) v[i].max = (v[j].max > v[j^1].max)? v[j].max: v[j^1].max;
}
 
int get_max(seg_node v[], int k, int l, int r)
{
	int tmp[2];
	if (v[k].right < l || v[k].left > r) return inf;
	else if (v[k].left >= l && v[k].right <= r) return v[k].max;
	else {
		tmp[0] = get_max(v, k << 1, l, r);
		tmp[1] = get_max(v, (k << 1) ^ 1, l, r);
		return (tmp[0] < tmp[1])? tmp[1]: tmp[0];
	}
}

int BS_left(seg_node v[], int k, int l, int r, int x)
{
	int tmp;
	if (v[k].max < x || v[k].right < l || v[k].left > r) return r + 1;
	else if (v[k].left == v[k].right) return v[k].left;
	else {
		tmp = BS_left(v, k << 1, l, r, x);
		if (tmp <= r) return tmp;
		else return BS_left(v, (k << 1) ^ 1, l, r, x);
	}
}

int BS_right(seg_node v[], int k, int l, int r, int x)
{
	int tmp;
	if (v[k].max < x || v[k].right < l || v[k].left > r) return r + 1;
	else if (v[k].left == v[k].right) return v[k].left;
	else {
		tmp = BS_right(v, (k << 1) ^ 1, l, r, x);
		if (tmp <= r) return tmp;
		else return BS_right(v, k << 1, l, r, x);
	}
}

int main()
{
	int i, N, Q, A;
	seg_node v[400001];
	scanf("%d %d", &N, &Q);
	init_node(v, 1, 1, N);
	for (i = 1; i <= N; i++) {
		scanf("%d", &A);
		update_node(v, i, A);
	}
	
	int t, x, y;
	for (i = 1; i <= Q; i++) {
		scanf("%d %d %d", &t, &x, &y);
		if (t == 1) update_node(v, x, y);
		else if (t == 2) printf("%d\n", get_max(v, 1, x, y));
		else printf("%d\n", BS_left(v, 1, x, N, y));
	}
	fflush(stdout);
	return 0;
}