// Aizu Vol0 0049: Blood Groups
// 2017.7.29

#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>

char *gets(char *);

#define MAX 50

int blood[MAX + 5];
int cnt[5];

int main()
{
	char buf[100], *p;
	int k, t;

	for (k = 0; k <= MAX; k++) blood[k] = 0;
	for (k = 1; k < 5; k++) cnt[k] = 0;
	while (gets(buf) != NULL) {
		p = buf;

		while (isspace(*p)) p++;
		k = *p++ - '0';
		if (isdigit(*p)) k = k * 10 + (*p++ - '0');

		while (isspace(*p)) p++;  // ','
		p++;

		while (isspace(*p)) p++;
		if (*p == 'A' && *(p + 1) == 'B') t = 3;
		else if (*p == 'B') t = 2;
		else if (*p == 'O') t = 4;
		else t = 1;

		if (blood[k] == 0) {
			blood[k] = t;
			cnt[t]++;
		} else if (blood[k] != t) while (1);
	}
	for (k = 1; k < 5; k++) printf("%d\n", cnt[k]);
	return 0;
}