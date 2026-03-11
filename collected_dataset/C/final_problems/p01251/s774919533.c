// AOJ 2132: Left Hand Rule
// 2017.12.13 bal4u@uu

#include <stdio.h>
#include <string.h>

char h[101][101], v[101][101];
char mk[101][101][4];
int mv[4][2] = {{-1,0},{0,1},{1,0},{0,-1}};

char buf[40], *p;
int getint()
{
	int n = 0;
	while (*p >= '0') n = (n<<3) + (n<<1) + (*p++ & 0xf);
	p++;
	return n;
}

int main()  
{  
	int W, H, N, x, y, i, x1, y1, x2, y2, x3, y3, d, ans;  

    while (fgets(p=buf, 30, stdin) && *p != '0') {
		W = getint(), H = getint(), N = getint();

		memset(h, 0, sizeof(h));
		memset(v, 0, sizeof(v));
		for (x = 0; x < W; x++) v[x][0] = v[x][H] = 1;
		for (y = 0; y < H; y++) h[0][y] = h[W][y] = 1;

		for (i = 0; i < N; i++) {
			fgets(p=buf, 30, stdin);
			x1 = getint(), y1 = getint(), x2 = getint(), y2 = getint();
			if (x1 == x2) {
				if (y1 > y2) y = y1, y1 = y2, y2 = y;
				for (y = y1; y < y2; y++) h[x1][y] = 1;
            } else {
				if (x1 > x2) x = x1, x1 = x2, x2 = x;
				for (x = x1; x < x2; x++) v[x][y1] = 1;
            }  
        }

		fgets(p=buf, 40, stdin);
		x1 = getint(), y1 = getint(), x2 = getint(), y2 = getint();
		x3 = getint(), y3 = getint();
		x = x1; if (x2 < x) x = x2;
		y = y1; if (y2 < y) y = y2;
		if (x1 == x2) { if (x1 == 0) d = 2; else x--, d = 0; }
		else          { if (y1 == 0) d = 1; else y--, d = 3; }

		memset(mk, 0, sizeof(mk));
		ans = 1;
		while (1) {
			if (x == x3 && y == y3) break;

			if (mk[x][y][d]) { ans = -1; break; }
			mk[x][y][d] = 1;

			d += 3; if (d >= 4) d -= 4;
			for (i = 0; i < 4; i++) {
				x2 = x + mv[d][0], y2 = y + mv[d][1];
                if (x2 >= 0 && x2 < W && y2 >= 0 && y2 < H) {
					if (d == 0 && !h[x  ][y] || d == 1 && !v[x][y+1] ||
						d == 2 && !h[x+1][y] || d == 3 && !v[x][y  ]) {	x = x2, y = y2; break; }
				}
                if (++d == 4) d = 0;
            }
            ans++;
        }
        if (ans < 0) puts("Impossible");
		else printf("%d\n", ans);
    }
    return 0;
}