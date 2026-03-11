#include<stdio.h>
#include<string.h>
#include<stdlib.h>

int main() {
	char* s = (char*)malloc(50001);
	char* top = gets(s);
	char* bottom = top + strlen(s) - 1;
	int count = 0;
	while (top < bottom) {
		if (*top == *bottom) {
			top++; bottom--; continue;
		}
		if (*top == 'x') {
			top++; count++; continue;
		}
		if (*bottom == 'x') {
			bottom--; count++; continue;
		}
		printf("-1");
		free(s);
		return 0;
	}
	printf("%d", count);
	free(s);
	return 0;
}
