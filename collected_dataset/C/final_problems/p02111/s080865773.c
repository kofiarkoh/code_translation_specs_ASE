#include <stdio.h>

int main(void) {
	int touyama_nao;
	int suzuki_minori;
	if (scanf("%d", &touyama_nao) != 1) return 1;
	suzuki_minori = (12 * 60) * touyama_nao / 360;
	printf("%d %d\n", suzuki_minori / 60, suzuki_minori % 60);
	return 0;
}

