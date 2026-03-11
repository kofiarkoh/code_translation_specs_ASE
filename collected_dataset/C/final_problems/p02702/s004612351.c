#include <stdbool.h>
#include <stdio.h>
#include <string.h>
typedef long long ll;
int main() {
	char s[200000];
	ll cnt = 0;
	scanf("%s", s);
	int len = strlen(s);
	ll sum;
	for (int i = 2; i < len; i++) {
		sum = (s[i - 2] - '0') * 100 + (s[i - 1] - '0') * 10 + s[i] - '0';
		for (int j = i + 1; j < len; j++) {
			sum = sum * 10 + (s[j] - '0');
			if (sum % 2019 == 0) {
				cnt++;
			}
			// printf("%lld\n", sum);
			sum %= 2019;
		}
	}
	printf("%lld\n", cnt);
	return 0;
}