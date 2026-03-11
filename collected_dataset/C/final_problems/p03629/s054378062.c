#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char shortage(char *str) {
	int i, alpha[26] = {0}, count = 0;
	for (i = 0; str[i] != '\0'; i++) {
		if (alpha[str[i] - 'a'] == 0) {
			alpha[str[i] - 'a'] = 1;
			count++;
		}
		if (count == 26)
			return '\0';
	}
	for (i = 0; i < 26; i++) {
		if (alpha[i] == 0)
			return 'a' + i;
	}
	return '\0';
}

int find(char *str, char c) {
	int i;
	for (i = 0; str[i] != '\0'; i++) {
		if (str[i] == c)
			return i;
	}
	return -1;
}

char A[200001];

int main(void) {
	int i, j, k, index;
	scanf("%s", A);

	char ans = shortage(A);
	if (ans != '\0') {
		printf("%c", ans);
		return 0;
	}

	for (i = 'a'; i <= 'z'; i++) {
		index = find(A, i);
		ans = shortage(A + index + 1);
		if (ans != '\0') {
			printf("%c%c",i, ans);
			return 0;
		}
	}

	for (i = 'a'; i <= 'z'; i++) {
		index = find(A, i);
		for (j = 'a'; j <= 'z'; j++) {
			index += find(A + index, j);
			ans = shortage(A + index + 1);
			if (ans != '\0') {
				printf("%c%c%c", i, j, ans);
				return 0;
			}
		}
	}

	for (i = 'a'; i <= 'z'; i++) {
		index = find(A, i);
		for (j = 'a'; j <= 'z'; j++) {
			index += find(A + index, j);
			for (k = 'a'; k <= 'z'; k++) {
				index += find(A + index, k);
				ans = shortage(A + index + 1);
				if (ans != '\0') {
					printf("%c%c%c%c", i, j, k, ans);
					return 0;
				}
			}
		}
	}

	return 0;
}