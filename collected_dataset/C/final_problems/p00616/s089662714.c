#include <stdio.h>
#include <string.h>

int main()
{
	int n, h, i, j, k;
	int x, y, z;
	int a, b, sum;
	//static char cube[501][501][501] = {0};
	int X[200], Y[200], Z[200];
	char buf[20];
	int count;

	while (1) {
		scanf("%d %d", &n, &h);
		if (n == 0 && h == 0)
			break;
		//for (x = 0; x <= n; x++) {
		//for (y = 0; y <= n; y++) {
		//for (z = 0; z <= n; z++) {
		//	cube[x][y][z] = 0;
		//}}}
		for (i = 0; i < h; i++) {
			scanf("%s %d %d", buf, &a, &b);
			if (!strcmp(buf, "xy")) {
				X[i] = a;
				Y[i] = b;
				Z[i] = 0;
			} else if (!strcmp(buf, "yz")) {
				X[i] = 0;
				Y[i] = a;
				Z[i] = b;
			} else if (!strcmp(buf, "xz")) {
				X[i] = a;
				Y[i] = 0;
				Z[i] = b;
			}
		}
		count = 0;
		for (i = 1; i < h; i++) {
			for (j = 0; j < i; j++) {
				if ( (X[i] == X[j] || X[i] * X[j] == 0) &&
					 (Y[i] == Y[j] || Y[i] * Y[j] == 0) && 
					 (Z[i] == Z[j] || Z[i] * Z[j] == 0) ) {
						 count++;
						 break;
				}
			}
		}
		printf("%d\n", n * n * n - n * h + count);
	}
	return 0;
}