#include <stdio.h>

int main(void)
{
	char s[101], t[101];

	scanf("%s %s", &s, &t);

	printf("%s%s", &t, &s);//T→Sの順に表示する

	return 0;
}