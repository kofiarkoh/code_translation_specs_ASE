#include <stdio.h>
int main(void)
{
	char s[26];
	int cnt[26] = {0};
	int i;
	scanf("%s", s);
	i = 0;
	while (s[i] != '\0') {
		// printf("%c", s[i]);
		cnt[s[i++] - 'a']++;
	}
	// putchar('\n');
	// for (i = 0; i < 26; i++)
	// 	printf("%c : %d\n", 'a' + i, cnt[i]);
	i = 0;
	while (cnt[i++] <= 1);
	if (i <= 26)
		puts("no");
	else
		puts("yes");
	return 0;
}