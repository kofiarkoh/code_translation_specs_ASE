#include <stdio.h>

int main() {

	int x, y, s, i=1, j,k=0;
	int answer[100000] = { 0 };

	while (i) {
		scanf("%d %d %d", &x, &y, &s);
		if (x == 0 && y == 0 && s == 0)
			break;
		for (i = 1; i < s; i++){
			for (j = 1; j <s; j++){
				if (s == i*(100 + x) / 100 + j*(100 + x) / 100) {
					if (answer[k] < i*(100 + y) / 100 + j*(100 + y) / 100) {
						answer[k] = i*(100 + y) / 100 + j*(100 + y) / 100;
					}
				}
			}
		}
		k++;
	}

	for (i = 0; i < k; i++)
		printf("%d\n", answer[i]);
	return 0;
}