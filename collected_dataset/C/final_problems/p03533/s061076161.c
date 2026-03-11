#include <stdio.h>
#include <string.h>

char str[55];
// AKIHABARA
// KIHBR
// 2^4

char target[][30] = {
	"AKIHABAR",
	"AKIHABARA",
	"AKIHABR",
	"AKIHABRA",
	"AKIHBAR",
	"AKIHBARA",
	"AKIHBR",
	"AKIHBRA",
	"KIHABAR",
	"KIHABARA",
	"KIHABR",
	"KIHABRA",
	"KIHBAR",
	"KIHBARA",
	"KIHBR",
	"KIHBRA"
};

int main()
{
	scanf("%s", str);

	int i = 0;
	while (i < 16 && strcmp(target[i], str)) {
		i++;
	}

	if (i == 16) {
		puts("NO");
	}
	else {
		puts("YES");
	}

	return 0;
}
