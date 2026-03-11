#include <stdio.h>
#include <string.h>

int main(void) {
	char ch;

	scanf("%c", &ch);

	puts(strchr("aeiou", ch) ? "vowel" : "consonant");

	return 0;
}
