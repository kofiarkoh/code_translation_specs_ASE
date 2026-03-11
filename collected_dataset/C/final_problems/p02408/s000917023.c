#include <stdio.h>

struct card {
	char Mark;
	int Number;
};

int main() {
	int n = 0;
	scanf("%d", &n);

	struct card data[54];
	for (int i = 0; i < n; i++) {
		(void)getchar();
		scanf("%c %d", &data[i].Mark, &data[i].Number);
	}

	char Mark[4] = { 'S', 'H', 'C', 'D' };
	int flag = 0;
	for (int c = 0; c < 4; c++) {
		for (int x = 1; x <= 13; x++) {
			flag = 0;
			for (int i = 0; i < n; i++) {
				if (Mark[c] == data[i].Mark && x == data[i].Number) {
					flag = 1;
				}
			}

			if (flag == 0) {
				printf("%c %d\n", Mark[c], x);
			}
		}
	}
	printf("\n");

	return 0;
}