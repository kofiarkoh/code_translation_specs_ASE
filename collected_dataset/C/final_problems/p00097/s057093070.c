#include <stdio.h>
#include <string.h>

int comb(int N, int S, int num);

int n, s;
int memo[101][1001][101];

int main(void)
{
	int i, j;
	
	while (1){
		
		
		memset(memo, -1, sizeof(memo));
		
		
		scanf("%d %d", &n, &s);
		if (!n && !s)return 0;
		
		printf("%ld\n", comb(n, 0, 0));
		
		
	}
	
}

int comb(int N, int S, int num)
{
	int i, j;
	
	if (s == S && !N)return 1;
	
	if (num == 101 || !N) return 0;
	
	if (memo[N][S][num] > -1) return memo[N][S][num];
	
	
	return memo[N][S][num] = comb(N - 1, S + num, num + 1) + comb(N, S, num + 1);
	
}