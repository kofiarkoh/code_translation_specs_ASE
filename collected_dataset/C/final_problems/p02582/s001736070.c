#include <stdio.h>

#define MAX 5

int main()
{
	char s[MAX];
	int count;
	int m = MAX;

	scanf("%s", s);

	count = 0;
	for (int i = 0; i < 3; i++) {
		if (s[i] == 'R')
			count++;
		else {
			m = count;
			count = 0;
		}
	}

	if (count > m)
		m = count;

	printf("%d\n", m);

	return 0;
}