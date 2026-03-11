#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int calc(char *fst, size_t f, char *snd, size_t s, char *ans, size_t a) {
	int buf = 0;

	while (a > 0) {
		if (f) {
			f--;
			buf += fst[f] - '0';
		}
		if (s) {
			s--;
			buf += snd[s] - '0';
		}
		a--;
		if ((ans[a] - '0') != (buf % 10)) {
			return 0;
		}
		buf /= 10;
	}
	return 1;
}

int main(void) {
	char in[130];
	char *first, *second, *answer;
	size_t f_len, s_len, a_len;
	char cf[130], cs[130], ca[130];
	const char *candidate, *c;
	int i;

	while (scanf("%s ", in) != EOF) {
		first = strtok(in, "+");
		second = strtok(NULL, "=");
		answer = strtok(NULL, "\n");

		f_len = strlen(first);
		s_len = strlen(second);
		a_len = strlen(answer);

		if (f_len > 1 && first[0] == 'X') {
			candidate = "123456789";
		} else if (s_len > 1 && second[0] == 'X') {
			candidate = "123456789";
		} else if (a_len > 1 && answer[0] == 'X') {
			candidate = "123456789";
		} else {
			candidate = "0123456789";
		}

		for (c = candidate; *c; c++) {
			for (i = 0; first[i]; i++) {
				cf[i] = first[i] != 'X' ? first[i] : *c;
			}
			cf[i] = '\0';
			for (i = 0; second[i]; i++) {
				cs[i] = second[i] != 'X' ? second[i] : *c;
			}
			cs[i] = '\0';
			for (i = 0; answer[i]; i++) {
				ca[i] = answer[i] != 'X' ? answer[i] : *c;
			}
			ca[i] = '\0';

			if (calc(cf, f_len, cs, s_len, ca, a_len)) {
				printf("%c\n", *c);
				c = NULL;
				break;
			}
		}
		if (c) {
			puts("NA");
		}
	}
	return 0;
}