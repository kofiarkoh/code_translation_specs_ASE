#include<stdio.h>
#include<string.h>
int main(void) {
	char s[4] = "ABC";
	gets(s);
	if (s[1] =='B') {
		printf("ARC");
	}
	else {
		printf("ABC");
	}
	return 0;
}