#include <stdio.h>

typedef struct node_t {
	int val;
	struct node_t *next;
} node;

node* new_node(int val) {
	static int it = 0;
	static node pool[200000];
	pool[it] = (node) { val, NULL };
	return &pool[it++];
}

node* st[1000];
node* en[1000];

int main(void)
{
	int n, q, com, t, x;
	node *tmp;
	scanf("%d%d", &n, &q);
	while (q--) {
		scanf("%d%d", &com, &t);
		if (com == 0) {
			scanf("%d", &x);
			tmp = new_node(x);
			if (!st[t]) st[t] = en[t] = tmp;
			else en[t]->next = tmp, en[t] = tmp;
		}
		else if (com == 1) {
			if (st[t]) printf("%d\n", st[t]->val);
		}
		else {
			if (st[t]) st[t] = st[t]->next;
		}
	}
	return 0;
}

