// AOJ 2534: Dictionary
// 2017.10.7 bal4u@uu

#include <stdio.h>
#include <string.h>

#define SIZE 26

char wd[502][12];
char mk[SIZE][SIZE];

int  q[SIZE], top, tail;
int  count[SIZE];

void push(int n) { q[tail++] = n; }
int  pop(int *n) { if (top == tail) return 0; *n = q[top++]; return 1; }
int topological_sort(int size)
{
    int i, j, k;

    memset(count, 0, sizeof(count));
    top = tail = 0;
    for (i = 0; i < size; i++) for (j = 0; j < size; j++)
        if (mk[i][j]) count[j]++;

    for (i = 0; i < size; i++) {
        if (count[i] == 0) push(i);
    }

    k = 0;
    while (pop(&i)) {
        for (j = 0; j < size; j++) if (mk[i][j])
			if (--count[j] == 0) push(j);
		k++;
    }
	return k;
}

int main()
{
	int n, i, a, b;
	char *p, *q;

	while (scanf("%d", &n) && n > 0) {
		memset(mk, 0, sizeof(mk));
		for (i = 0; i < n; i++) scanf("%s", wd[i]);
		if (n == 1) { puts("yes"); continue; }
		for (i = 1; i < n; i++) {
			q = wd[i-1], p = wd[i];
			while (*p && *q == *p) q++, p++;
			if (!*q) continue;
			if (!*p) goto NO;
			a = *p - 'a', b = *q - 'a';
			if (mk[b][a]) goto NO;
			mk[a][b] = 1;
		}

		if (topological_sort(SIZE) == SIZE) puts("yes");
		else {
NO:			puts("no");
		}
	}
	return 0;
}