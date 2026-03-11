#include<stdio.h>
#include<string.h>

int main() {
	char s[5000]; char buff[5000];
	int K;
	char top[5][5000] = { {"{"},{"{"},{"{"},{"{"},{"{"} };
	scanf("%s", s);
	scanf("%d", &K);
	for (unsigned int i = 1; i <= strlen(s); i++) {
		for (unsigned int j = 0; j <= strlen(s) - i; j++) {
			strncpy(buff, s + j, i);
			buff[i] = '\0';
			for (int k = 0; k <= 4; k++) {
				if (!strcmp(top[k], buff))break;
				else if (strcmp(top[k], buff) > 0) {
					for (int h = 4; h > k; h--) {
						strcpy(top[h], top[h - 1]);
					}
					strncpy(top[k], buff, i);
					break;
				}
			}
		}
	}
	printf("%s\n", top[K - 1]);

	return 0;
}