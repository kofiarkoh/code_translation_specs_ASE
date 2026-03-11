#include<stdio.h>
#include<stdlib.h>

#define max(a,b) ((a)>(b))?(a):(b)
#define min(a,b) ((a)>(b))?(b):(a)

typedef long long ll;
typedef unsigned long long ull;

int main(void)
{
	int n;
	scanf("%d", &n);
	int* a = calloc(n, sizeof(int));
	int* b = calloc(n, sizeof(int));
	for (size_t i = 0; i < n; i++)
	{
		scanf("%d", a + i);
	}
	for (size_t i = 0; i < n; i++)
	{
		scanf("%d", b + i);
	}

	for (size_t i = 0; i < n; i++)
	{
		int a_ = a[i] ^ a[(i + 1)%n];
		int b_ = b[0] ^ b[1];
		int ist = i;
		int flg = 0;
		size_t j = i + 1;
		size_t lc = 1;
		while (a_==b_)
		{
			a_ = a[j%n] ^ a[(j+1)%n];
			b_ = b[lc] ^ b[lc + 1];

			j++; lc++;
			if (lc==n-1)
			{
				printf("%d %d\n", ist, a[ist] ^ b[0]);
				break;
			}
		}
	}
}