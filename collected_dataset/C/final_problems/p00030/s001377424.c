#include <stdio.h>
#include <string.h>


int val[10];
int s;

int getWay(int x, int n, int sum)
{
	int ret = 0;
	int i;
	
	if (n == 0){
		if (sum == s){
			return (1);
		}
		return (0);
	}
	
	for (i = x; i < 10; i++){
		if (val[i] == 0 && sum + i <= s){
			val[i] = 1;
			ret += getWay(i, n - 1, i + sum);
			val[i] = 0;
		}
	}
	
	return (ret);
}

int main(void)
{
	while (1){
		int n;
		
		scanf("%d%d", &n, &s);
		memset(val, 0, sizeof(val));
		
		if (n == 0 && s == 0){
			break;
		}
		
		printf("%d\n", getWay(0, n, 0));
	}
	
	return (0);
}