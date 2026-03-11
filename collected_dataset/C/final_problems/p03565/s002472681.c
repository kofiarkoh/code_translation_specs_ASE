#include<stdio.h>
#include<string.h>
int main()
{
	int j, i, k, flag = 1, tt = 0;
	char S[60], T[60], temp[60];
	gets(S);
	gets(T);

	for (i = strlen(S) - strlen(T); i >= 0; i--)
	{
		strcpy(temp, S);
		strcpy(temp + i, T);
		for (j = 0; j < strlen(S); j++)
		{
			if (S[j] != '?')
			{
				if (S[j] != temp[j])
				{
					break;
				}
			}
			if (j == strlen(S) - 1)
			{
				flag = 0;
				for (i = 0; i < strlen(temp); i++)
					if (temp[i] == '?') printf("a");
					else printf("%c", temp[i]);
				goto end;
			}
		}
	}

end:
	if (flag == 1)
		printf("UNRESTORABLE");


}