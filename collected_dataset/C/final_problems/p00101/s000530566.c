#include <stdio.h>
#include <string.h>

int main(void) {
	int n;
	char in[1024], *ptr;

	scanf("%d ", &n);

	while (n--) {
		fgets(in, 1023, stdin);
		ptr = strstr(in, "Hoshino");
		while (ptr != NULL) {
			ptr[6] = 'a';
			ptr = strstr(ptr, "Hoshino");
		}
		printf("%s", in);
	}
	return 0;
}