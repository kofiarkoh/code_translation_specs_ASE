#include <stdio.h>
#include <string.h>

#define MAXN 200000

int main() {
	char s[MAXN+1];
	long ct[26];
	while (scanf("%s", s)==1) {
		memset(ct, 0, sizeof(ct));
		for (int i=0; s[i]; i++)
			ct[s[i]-'a']++;
		long r = 1;
		for (int i=0; i<26; i++)
			for (int j=i+1; j<26; j++)
				r += ct[i] * ct[j];
		printf("%ld\n", r);
	}
}

