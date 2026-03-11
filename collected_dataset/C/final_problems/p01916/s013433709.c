#include <stdio.h>

int main(void) {
	char S[1024];
	int count[26] = {0};
	int i;
	int asumikana = 0;
	if (scanf("%1023s", S) != 1) return 1;
	for (i = 0; S[i] != '\0'; i++) {
		int which = S[i] - 'a';
		if (0 <= which && which < 26) count[which]++;
	}
	for (i = 0; i < 26; i++) {
		if (count[i] % 2 != 0) asumikana++;
	}
	printf("%d\n", asumikana / 2);
	return 0;
}

