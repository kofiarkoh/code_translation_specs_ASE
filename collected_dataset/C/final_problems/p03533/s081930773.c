#include <stdio.h>
#include <string.h>
int main()
{
	char s[100];
	/*while (1)
	{*/
		gets(s);
		if (strcmp(s, "AKIHABARA") == 0 || strlen(s) >= 9)
			printf("NO\n");
		else
		{
			int count = 0;
			for (int i = 0; i < strlen(s); i++)
			{
				if (s[i] == 'A')
					count++;
			}
			if (count == 0)
			{
				if (strcmp(s, "KIHBR") == 0)
					printf("YES\n");
				else
					printf("NO\n");
			}
			else if (count == 1)
			{
				if (strcmp(s, "AKIHBR") == 0 || strcmp(s, "KIHABR") == 0 || strcmp(s, "KIHBAR") == 0
					|| strcmp(s, "KIHBRA") == 0)
					printf("YES\n");
				else
					printf("NO\n");
			}
			else if (count == 2)
			{
				if (strcmp(s, "AKIHABR") == 0 || strcmp(s, "AKIHABR") == 0 || strcmp(s, "AKIHBAR") == 0
					|| strcmp(s, "AKIHBRA") == 0 || strcmp(s, "KIHABAR") == 0 || strcmp(s, "KIHABRA") == 0
					|| strcmp(s, "KIHBARA") == 0)
					printf("YES\n");
				else
					printf("NO\n");
			}
			else if (count == 3)
			{
				if (strcmp(s, "AKIHABAR") == 0 || strcmp(s, "AKIHABRA") == 0 || strcmp(s, "KIHABARA") == 0)
					printf("YES\n");
				else
					printf("NO\n");
			}
			else
				printf("NO\n");
		}
	//}
	return 0;
}