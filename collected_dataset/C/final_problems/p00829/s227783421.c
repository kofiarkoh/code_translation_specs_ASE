// AOJ 1250: Leaky Cryptography
// 2017.10.16 bal4u@uu

#include <stdio.h>

unsigned v[9];

int main()
{
	int n, i, j;
	unsigned sum, ans;

	scanf("%d", &n);
	while (n-- > 0) {
		for (i = 0; i < 9; i++) scanf("%x", v+i);

		ans = 0;
		for (i = 0; i < 32; i++) {
			for (sum = 0, j = 0; j < 8; j++) sum += v[j] ^ ans;
			if ((sum & (1<<(i+1)-1)) != ((v[8] ^ ans) & (1<<(i+1)-1)))
				ans |= (1 << i);
		}
		printf("%x\n",ans);
    }
	return 0;
}