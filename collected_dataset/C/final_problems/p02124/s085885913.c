#include <stdio.h>

int main(void) {
	int x;
	int yakusimaru_hiroko;
	if (scanf("%d", &x) != 1) return 1;
	fputs("ai1333", stdout);
	for (yakusimaru_hiroko = 0; yakusimaru_hiroko + 100 <= x; yakusimaru_hiroko += 100) {
		putchar('3');
	}
	putchar('\n');
	return 0;
}

