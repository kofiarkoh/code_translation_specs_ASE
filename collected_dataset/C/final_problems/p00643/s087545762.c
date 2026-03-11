// AOJ 1057: Rolling Dice
// 2017.10.21 bal4u@uu

#include <stdio.h>
#include <string.h>

#define QSIZE 5000000

#define PARENT(i) ((i)>>1)
#define LEFT(i)   ((i)<<1)
#define RIGHT(i)  (((i)<<1)+1)

typedef struct { char r, c, d[3]; int t; } QUE;
QUE que[QSIZE+2];
int qsize;

void min_heapify(int i)
{
	int l, r, min;

	l = LEFT(i), r = RIGHT(i);
	if (l < qsize && que[l].t < que[i].t) min = l; else min = i;
	if (r < qsize && que[r].t < que[min].t) min = r;
	if (min != i) {
		QUE t = que[i]; que[i] = que[min]; que[min] = t;
		min_heapify(min);
	}
}

void deq(void)
{
//	if (qsize == 0) return 0;
//	*a = que[0].a, *b = que[0].b, *c = que[0].c;
	que[0] = que[--qsize];
	min_heapify(0);
}

void enq(void)
{
	int i, min;

	i = qsize++;
//	que[i].t = t, que[i].a = a, que[i].b = b, que[i].c = c;
	while (i > 0 && que[min = PARENT(i)].t > que[i].t) {
		QUE tt = que[i]; que[i] = que[min]; que[min] = tt;
		i = min;
	}
}

char map[11][11];
int mk[11][11][6][6];
int mv[4][2] =  {{-1,0},{0,1},{1,0},{0,-1}};
int rot[4][3] = {{2,6,3},{4,2,1},{5,1,3},{3,2,6}};

int main()
{
	int h, w, sr, sc, gr, gc, i, r, c, t, r2, c2, t2, ans;
	char dice[7], dice2[7];

	while (scanf("%d%d", &h, &w) && h > 0) {
		memset(mk, 0, sizeof(mk));
		for (r = 0; r < h; r++) for (c = 0; c < w; c++) scanf("%d", &i), map[r][c]= i;
		scanf("%d%d%d%d", &sr, &sc, &gr, &gc);
		qsize = 0;
		que[0].r = sr, que[0].c = sc, que[0].t = 0, que[0].d[0] = 1, que[0].d[1] = 2, que[0].d[2] = 3;
		ans = 0x7fffffff; enq();
		while (qsize > 0) {
			r = que[0].r, c = que[0].c, t = que[0].t;
			memcpy(dice+1, que[0].d, 3); for (i = 4; i <= 6; i++) dice[i] = 7 - dice[7-i]; deq();
			if (r == gr && c == gc) { ans = t; break; }
			for (i = 0; i < 4; i++) {
				r2 = r + mv[i][0], c2 = c + mv[i][1];
				if (r2 < 0 || r2 >= h || c2 < 0 || c2 >= w) continue;
				dice2[1] = dice[rot[i][0]],	dice2[2] = dice[rot[i][1]], dice2[3] = dice[rot[i][2]];
				t2 = t + (7 - dice2[1])*map[r2][c2];
				if (mk[r2][c2][dice2[1]][dice2[2]] == 0 || mk[r2][c2][dice2[1]][dice2[2]] > t2) {
					mk[r2][c2][dice2[1]][dice2[2]] = t2;
					que[qsize].r = r2, que[qsize].c = c2, que[qsize].t = t2;
					memcpy(que[qsize].d, dice2+1, 3); enq();
				}
			}
		}
		printf("%d\n", ans);
	}
	return 0;
}