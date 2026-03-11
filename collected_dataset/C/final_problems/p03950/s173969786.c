#include <stdio.h>

typedef struct {
	int x, y;
} point;

int cmp1(const void *a, const void *b) {
	point c = *(point *)a;
	point d = *(point *)b;
	if (c.x < d.x) return -1;
	else if (c.x == d.x) return 0;
	else return 1;
}
int cmp2(const void *a, const void *b) {
	point c = *(point *)a;
	point d = *(point *)b;
	if (c.y < d.y) return -1;
	else if (c.y == d.y) return 0;
	else return 1;
}

int main(void) {
	int H, i, L1, L2, N, W;
	point p[300010];
	scanf("%d%d%d", &W, &H, &N);
	for (i=0;i<N;i++) {
		scanf("%d%d", &p[i].x, &p[i].y);
	}
	if (N>4) {
		qsort(p, N, sizeof(point), cmp1);
		L1 = p[1].x - p[0].x;
		for (i=0;i<N-1;i++) {
			L1 = (L1<p[i+1].x-p[i].x) ? p[i+1].x-p[i].x : L1;
		}
		qsort(p, N, sizeof(point), cmp2);
		L2 = p[1].y - p[0].y;
		for (i=1;i<N-1;i++) {
			L2 = (L2<p[i+1].y-p[i].y) ? p[i+1].y-p[i].y : L2;
		}
		
		printf("%d\n", (H+L1<W+L2) ? (W+L2)*2 : (H+L1)*2);
	} else {
		int EL, ER, ET, EB, L, R, T, B;
		EL = 0; ER = W;
		EB = 0; ET = H;
		for (i=0;i<N;i++) {
			L = p[i].x;
			R = W - p[i].x;
			B = p[i].y;
			T = H - p[i].y;
			if (L <= R && L <= B && L <= T) {
				if (EL<p[i].x) EL = p[i].x;
			} else if (R <= L && R <= B && R <= T) {
				if (ER>p[i].x) ER = p[i].x;
			} else if (T <= L && T <= R && T <= B) {
				if (ET>p[i].y) ET = p[i].y;
			} else {
				if (EB<p[i].y) EB = p[i].y;
			}
		}
		printf("%d\n", 2*((ER-EL)+(ET-EB)));
	}
	return 0;
}