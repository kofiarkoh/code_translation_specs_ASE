#include <stdio.h>
#include <string.h>
#define min(X, Y) ((X) < (Y) ? (X) : (Y))

int ch_string(char str[], char ch);

int n;
char str[55];
int ch[30];

int main()
{
	for (int i = 0; i < 30; i++) {
		ch[i] = 100000;
	}

	scanf("%d", &n);
	for (int i = 0; i < n; i++) {
		scanf("%s", str);

		for (int j = 'a'; j <= 'z'; j++) {
			ch[j - 'a'] = min(ch[j - 'a'], ch_string(str, j));
		}
	}

	for (int i = 'a'; i <= 'z'; i++) {

		if (ch[i - 'a'] < 100000) {
			for (int j = 0; j < ch[i - 'a']; j++) {
				putchar(i);
			}
		}
	}

	return 0;
}

int ch_string(char str[], char ch)
{
	int res = 0;

	while (*str != '\0') {
		if (*str == ch) {
			res++;
		}
		str++;
	}

	return res;
}
