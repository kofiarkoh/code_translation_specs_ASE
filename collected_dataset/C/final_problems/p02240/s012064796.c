#include <stdio.h>
#define MAX 1000000

int data[MAX];
void insert() {
	int i;
	for (i = 0; i < MAX; ++i) {
		data[i] = i;
	}
}
int find(int a) {
	if (a == data[a]) {
		return a;
	}
	return data[a] = find(data[a]);
}
int search(int a, int b) {
	if (find(a) == find(b)) {
		return 1;
	} else {
		return 0;
	}
}
void swap(int a, int b) {
	a = find(a);
	b = find(b);
	if (a == b) return;
	data[b] = a;
}
int main() {
	int n, m, q, i, j, k;

	insert();
	scanf("%d %d", &n, &m);
	for (i = 0 ; i < m ; ++i) {
		scanf("%d %d", &j, &k);
		swap(j, k);
	}
	scanf("%d", &q);
	for (i = 0 ; i < q ; i++) {
		scanf("%d %d", &j, &k);
		if (search(j, k)) {
			printf("yes\n");
		} else {
			printf("no\n");
		}
	}
	return 0;
}

