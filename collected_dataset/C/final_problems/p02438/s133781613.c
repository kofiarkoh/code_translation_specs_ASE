#include <stdio.h>
#include <stdlib.h>

struct node_t {
	int x;
	struct node_t* next;
};

int main(void) {
	int n, q;
	struct node_t *head[1024], **tail[1024];
	int i;
	if (scanf("%d%d", &n, &q) != 2) return 1;
	for (i = 0; i < n; i++) {
		head[i] = NULL;
		tail[i] = &head[i];
	}
	for (i = 0; i < q; i++) {
		int query, s, t, x;
		if (scanf("%d", &query) != 1) return 1;
		switch (query) {
			case 0:
				if (scanf("%d%d", &t, &x) != 2) return 1;
				*tail[t] = malloc(sizeof(struct node_t));
				if (*tail[t] == NULL) return 1;
				(*tail[t])->x = x;
				(*tail[t])->next = NULL;
				tail[t] = &(*tail[t])->next;
				break;
			case 1:
				if (scanf("%d", &t) != 1) return 1;
				{
					struct node_t* itr = head[t];
					while (itr != NULL) {
						if (itr != head[t]) putchar(' ');
						printf("%d", itr->x);
						itr = itr->next;
					}
					putchar('\n');
				}
				break;
			case 2:
				if (scanf("%d%d", &s, &t) != 2) return 1;
				*tail[t] = head[s];
				if (head[s] != NULL) tail[t] = tail[s];
				head[s] = NULL;
				tail[s] = &head[s];
				break;
		}
	}
	for (i = 0; i < n; i++) {
		while (head[i] != NULL) {
			struct node_t* next = head[i]->next;
			free(head[i]);
			head[i] = next;
		}
	}
	return 0;
}

