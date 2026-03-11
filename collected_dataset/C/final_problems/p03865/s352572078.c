#include<stdio.h>
#include<string.h>
int main()
{
	char s[100005];
	scanf("%s", s);
	int i, n = strlen(s);
	int x = 0;
	for (i = 2; i < n - 1; i += 2)
		if (s[0] != s[i])
			x++;
	for (i = n - 3; i > 0; i -= 2)
		if (s[n - 1] != s[i])
			x++;
	if (n == 3 && s[0] != s[2])
		x++;
	if (x == 0)
		printf("Second\n");
	else if (n % 2 == 0)
		printf("Second\n");
	else
		printf("First\n");
	return 0;
}