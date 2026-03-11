#include <stdio.h>
#include <string.h>

int cmp(char *s1, char *s2)
{
	int cnt = 0;
	for (int i = 0; i < strlen(s1);i++)
	{
		if (s1[i] == s2[i]) cnt++;
	}
	if (cnt == strlen(s1)) return 1;
	else return 0;
}
int main(void)
{
	char str1[50]; int cnt = 0;
	scanf("%s", str1);
	if (strlen(str1) >= 9) printf("NO\n");
	else{
		char *a[15];
		a[0] = "KIHB"; a[1] = "AKIHBR"; a[2] = "KIHABR"; a[3] = "KIHBAR"; a[4] = "KIHBRA"; a[5] = "AKIHABR";
		a[6] = "AKIHBAR"; a[7] = "AKIHBRA"; a[8] = "KIHABAR"; a[9] = "KIHABRA"; a[10] = "KIHBARA"; a[11] = "AKIHABAR";
		a[12] = "AKIHABRA"; a[13] = "AKIHBARA"; a[14] = "KIHABARA";
		for (int i = 0; i < 15; i++)
		{
			if (cmp(a[i],str1)) {
				cnt = 1; break;
			}
		}
		if (cnt == 1) printf("YES\n");
		else printf("NO\n");
	}
	return 0;
}