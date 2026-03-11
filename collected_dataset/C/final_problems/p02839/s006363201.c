#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <math.h>

struct link {
	struct link *next;
	int val;
};

struct link *newval(struct link *prev, int val)
{
	if (prev) {
		for (struct link *p = prev; p; p = p->next) {
			if (p->val == val)
				return prev;
			if (p->val == -val)
				return prev;
		}
	}
	struct link *l = malloc(sizeof(struct link *));
	l->next = prev;
	l->val = val;
	return l;
}

struct link *update(struct link *me, struct link *prev, int diff)
{
	for (struct link *p = prev; p; p = p->next) {
		me = newval(me, p->val + diff);
		me = newval(me, p->val - diff);
	}
	return me;
}

int main(int argc, char **argv)
{
	int H, W;
	scanf("%d %d", &H, &W);
	int A[80][80], B[80][80];
	for (int h = 0; h < H; h++) {
		for (int w = 0; w < W; w++)
			scanf("%d", &A[h][w]);
	}
	int D[80][80];
	for (int h = 0; h < H; h++) {
		for (int w = 0; w < W; w++) {
			scanf("%d", &B[h][w]);
			// get diff
			D[h][w] = abs(A[h][w] - B[h][w]);
		}
	}
	struct link *p[80][80] = {0}; // NULL
	for (int h = 0; h < H; h++) {
		for (int w = 0; w < W; w++) {
			if (h == 0 && w == 0) {
				p[h][w] = newval(NULL, D[0][0]);
				continue;
			}
			if (h > 0)
				p[h][w] = update(p[h][w], p[h - 1][w], D[h][w]);
			if (w > 0)
				p[h][w] = update(p[h][w], p[h][w - 1], D[h][w]);
		}
	}
	int m = 80*80*80;
	for (struct link *l = p[H-1][W-1]; l; l = l->next) {
		fprintf(stderr, "%d\n", l->val);
		if (abs(l->val) < m)
			m = abs(l->val);
	}
	printf("%d\n", m);
	return 0;
}
