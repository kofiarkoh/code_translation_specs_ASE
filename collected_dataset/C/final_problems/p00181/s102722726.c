#include <stdio.h>

int main(void) {
	int m, n;
	int t[128];
	int i;

	while(1) {
		int top = 1500000, bottom = 0;
		scanf("%d %d", &m, &n);
		if(!m && !n)
			break;
		for(i = 0; i < n; i++)
			scanf("%d", &t[i]);

		while(top != bottom) {
			int med = (top + bottom) / 2;
			int tmp = 0, tmp2 = 0, tmp3 = 0;

			for(i = 0; i < n && tmp2 <= m; i++) {
				if(tmp + t[i] <= med) {
					tmp += t[i];
				}
				else {
					tmp = 0;
					tmp2++;
					i--;
				}
			}

			if(tmp2 < m)
				top = med;
			else
				bottom = med + 1;
		}

		printf("%d\n", bottom);
	}

	return 0;
}