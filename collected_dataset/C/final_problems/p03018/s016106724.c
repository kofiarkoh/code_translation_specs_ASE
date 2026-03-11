#include<stdio.h>
#include<stdlib.h>
#include<string.h>
int main()
{
	int i;
	int len;
	int check;
	int ans = 0;
	char(*S) = (char(*))malloc(200010 * sizeof(char));

	scanf("%s", S);
	len = strlen(S);
	for (i = 0;i < len;i++)
	{
		if (S[i] == 'A' &&S[i + 1] == 'B' && S[i + 2] == 'C')
		{
			S[i] ='B';
			S[i + 1] = 'C';
			S[i + 2] = 'A';
			ans++;
			if (i >= 3)i -= 2;
		}

	}

	printf("%d\n", ans);
	return 0;
}