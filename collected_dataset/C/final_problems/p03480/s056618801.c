#include <stdio.h>
#include <string.h>
int main() {
	char s[100010] = {};
	int ans = 100010, len, t;
	scanf("%s", s);
	len = strlen(s);
	for (int i = 1; i < len; i++) {
		if(s[i - 1] != s[i]) {
			t = (i > (len - i)) ? i : (len - i);
			ans = (ans < t) ? ans : t;
		}
	}
	printf("%d", ans);
	return 0;
}
