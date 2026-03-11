#include <stdio.h>
#include <string.h>
typedef long long ll;

ll n;
char str[100005];

ll win, lose;
ll g, p;

int main()
{
	scanf("%s", str);

	n = strlen(str);
	for (ll i = 0; i < n; i++) {
		if (g == p) {
			g++;

			if (str[i] == 'p') {
				lose++;
			}

		} else {
			p++;

			if (str[i] == 'g') {
				win++;
			}
		}
	}

	printf("%lld\n", win - lose);

	return 0;
}
