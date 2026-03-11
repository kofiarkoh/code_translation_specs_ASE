#include <stdio.h>
#include <string.h>

char prime[10001];
int m[10][1000][900];

int get_sum(int num, int sum, int n){
    
	int i, j;
    
    if (num == 0 && sum == 0){
        return (1);
	}
    if (num == 0 || sum <= 0 || sum < n){
        return (0);
    }
    
    if (num < 10 && sum < 1000 & n < 900){
        if (m[num][sum][n] == -1){
            m[num][sum][n] = (prime[n] ? get_sum(num - 1, sum - n, n + 1) : 0) + get_sum(num, sum, n + 1);
        }
        return (m[num][sum][n]);
    }
    return ((prime[n] ? get_sum(num - 1, sum - n, n + 1) : 0) + get_sum(num, sum, n + 1));
}

int main(void)
{
	int i, j;
	int n;
	int num, sum;
    
	for (i = 3; i <= 10000; i += 2){
		prime[i] = 1;
	}
	prime[2] = 1;
	
	for (i = 3; i <= 100; i += 2){
		if (prime[i] == 1){
			for (j = 3; j <= 10000 / i; j += 2){
				prime[i * j] = 0;
			}
		}
	}
    
    memset(m, -1, sizeof(m));
	while (1){
		scanf("%d%d", &sum, &num);
        
		if (num == 0 && sum == 0){
            break;
		}
		printf("%d\n", get_sum(num, sum, 2));
	}
	
	return (0);
}