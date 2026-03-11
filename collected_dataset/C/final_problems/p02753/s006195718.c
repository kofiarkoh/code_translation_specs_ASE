#include<stdio.h>
int main(void) {
	char s[100];
	scanf("%s", s);
	if (s[0] == s[1] && s[1] == s[2]) puts("No");
	else puts("Yes");
	return 0;
}