// AOJ 2723 Surface Area of Cubes
// 2018.2.7 bal4u
 
#include <stdio.h>
#include <string.h>

#define HSIZ 3011
typedef struct { int x, y, z; } HASH;
HASH hash[HSIZ+2], *hashend = hash+HSIZ;

int lookup(int x, int y, int z)
{
	long long h = ((long long)x << 32) + ((long long)y << 16) + z;
	HASH *p = hash + (int)(h % HSIZ);
	while (p->x) {
		if (p->x == x && p->y == y && p->z == z) return 1;
		if (++p == hashend) p = hash;
	}
	return 0;
}

void insert(int x, int y, int z)
{
	long long h = ((long long)x << 32) + ((long long)y << 16) + z;
	HASH *p = hash + (int)(h % HSIZ);
	while (p->x) {
		if (p->x == x && p->y == y && p->z == z) return;
		if (++p == hashend) p = hash;
	}
	p->x = x, p->y = y, p->z = z;
}

int dx[6] = {1,0,0,-1, 0, 0};
int dy[6] = {0,1,0, 0,-1, 0};
int dz[6] = {0,0,1, 0, 0,-1};

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	while (c < '0') c = getchar_unlocked();
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

int main()
{
	int a, b, c, n, x, y, z;
	int i, f, nx, ny, nz;
	long long ans;

	a = in(), b = in(), c = in(), n = in();
	ans = (long long)a*b + (long long)b*c + (long long)c*a;
	ans <<= 1;

	while (n--) {
		x = in(), y = in(), z = in();
	    f = 0;
		for (i = 0; i < 6; i++) {
			nx = x + dx[i], ny = y + dy[i], nz = z + dz[i];
			if (nx < 0 || nx >= a || ny < 0 || ny >= b ||
				nz < 0 || nz >= c || lookup(1+nx, ny, nz)) f--;
			else f++;
		}
		ans += f;
		insert(1+x, y, z);
	}
	printf("%lld\n", ans);
	return 0;
}
