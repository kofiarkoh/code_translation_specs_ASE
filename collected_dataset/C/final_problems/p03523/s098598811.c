#include <stdio.h>
#include <string.h>
int main() {
	char s[51];
	char okStr[16][10] = {"KIHBR",	 "KIHBRA",	 "KIHBAR",	 "KIHBARA",
						  "KIHABR",	 "KIHABRA",	 "KIHABAR",	 "KIHABARA",
						  "AKIHBR",	 "AKIHBRA",	 "AKIHBAR",	 "AKIHBARA",
						  "AKIHABR", "AKIHABRA", "AKIHABAR", "AKIHABARA"};
	scanf("%s", s);
	for (int i = 0; i < 16; i++) {
		if (strcmp(s, okStr[i]) == 0) {
			printf("YES\n");
			return 0;
		}
	}
	printf("NO\n");
	return 0;
}