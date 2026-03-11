#include <stdio.h>
#define MOD 1000000007
//N==4 M==3
int main()
{
	int i, n, m;
	scanf("%d%d", &n, &m);//長さ、1の個数

	long a[110000], ans = 0;
	for (i = 1; i < m; i++)
	{
		a[i] = 0;
	}
	a[0] = a[m] = 1; //a[0]=1, a[1]=0, a[2]=0, a[3]=1

	for (i = m; i <= n; i++)
	{
		ans = (ans * 2 + a[i]) % MOD;//ans = a[3]==1⇒ans = 1 * 2 + a[4] == 3⇐最終的なans
		a[i + 1] = (a[i] * 2 - a[i - m] + MOD) % MOD;// a[4] = a[3] * 2 - a[3 - 3] ==1
	}

	printf("%ld\n", ans);
}



