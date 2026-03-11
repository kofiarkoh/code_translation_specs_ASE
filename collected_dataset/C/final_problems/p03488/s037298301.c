#include<stdio.h>
#include<string.h>
#include<stdlib.h>
#include<math.h>

//for
#define REP(i,n) for(int i=0,i##_len=(int)(n);i<i##_len;i++)
#define REPS(i,n) for(int i=1,i##_len=(int)(n);i<=i##_len;i++)
#define RREP(i,n) for(int i=(int)(n)-1;i>=0;i--)
#define RREPS(i,n) for(int i=(int)(n);i>0;i--)

//printf
#define PRINTD(d) printf("%d\n",(d))
#define PRINTS(s) printf("%s\n",(s))

//memset
#define m0(s) memset(s,0,sizeof(s))
#define ml(s) memset(s,63,sizeof(s))
#define fill(s,c) memset(s,c,sizeof(s))

typedef long long ll;

char s[8001];
int x, y;
int cntX[4000] = { 0 }, cntY[4000] = { 0 };
int X = 0, Y = 0;
int idxX = -1, idxY = -1;
int cntT = 0;
int len;
int flag = 1;
int dpx[8000][8000] = { 0 };
int dpy[8000][16000] = { 0 };

int searchx(int i, int dis, int* cnt) {
	if (dpx[i][dis + 8000] != 0)return dpx[i][dis + 8000];
	if (i == 0) {
		//PRINTS("s");
		if (dpx[i][dis+8000])return 1;
		return 0;
	}
	dpx[i][dis + 8000] = (searchx(i - 1, dis + cnt[i-1], cnt))==1 || (searchx(i - 1, dis - cnt[i-1], cnt)==1);
	//printf("dis=%d, cnt[%d]=%d, %d\n", dis, i - 1, cnt[i - 1], dpx[i][dis + 8000]);
	if (!dpx[i][dis + 8000])dpx[i][dis + 8000] = -1;
	//printf("i=%d,dis+8000=%d\n", i, dis + 8000); PRINTD(dpx[i][dis + 8000]);
	return dpx[i][dis + 8000];
}
int searchy(int i, int dis, int* cnt) {
	if (dpy[i][dis + 8000] != 0)return dpy[i][dis + 8000];
	if (i == 0) {
		if (dpy[i][dis+8000])return 1;
		return 0;
	}
	dpy[i][dis + 8000] = (searchy(i - 1, dis + cnt[i-1], cnt)==1) || (searchy(i - 1, dis - cnt[i-1], cnt)==1);
	//printf("cnt[%d]=%d, %d\n", i, cnt[i], dpy[i][dis + 8000]);
	if (!dpy[i][dis + 8000])dpy[i][dis + 8000] = -1;
	return dpy[i][dis + 8000];
}

int main() {
	scanf("%s", s);
	scanf("%d %d", &x, &y);
	len = strlen(s);
	int cnt = 0;
	REP(i, len) {
		if (s[i] == 'T') {
			x -= cnt;
			break;
		}
		cnt++;
		if (i == len - 1) {
			X = len;
			if (X != x || y != 0)flag = 0;
		}
	}
	if (X != len) {
		for (int i = cnt; i < len; i++) {
			if (s[i] == 'T') {
				if (cntT) {
					idxX++;
				}
				else {
					idxY++;
				}
				cntT = 1 - cntT;
			}
			else {
				if (cntT) {
					cntY[idxY]++;
				}
				else {
					cntX[idxX]++;
				}
			}
		}
		//printf("%d %d\n", x, idxX);
		//printf("%d %d\n", y, idxY);
		dpx[0][8000] = 1;
		dpy[0][8000] = 1;
		if(idxX==-1){
			flag = (x == 0);
		}
		else if (searchx(idxX+1, x, cntX)!=1)flag = 0;
		if(idxY == -1) {
			flag = (y == 0);
		}else if (searchy(idxY+1, y, cntY)!=1)flag = 0;
	}
	//printf("x=%d,y=%d,X=%d,Y=%d\n", x, y, X, Y);
	//printf("%d<%d || %d!=%d\n", abs(X), abs(x), X%2, x%2);
	//printf("%d<%d || %d!=%d\n", abs(Y), abs(y), Y % 2, y % 2);
	//PRINTD(flag);
	if (flag) {
		PRINTS("Yes");
	}
	else {
		PRINTS("No");
	}
}