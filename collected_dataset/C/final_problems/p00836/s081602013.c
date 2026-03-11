#include <stdio.h>

#define MAX_PRIME	(10000)


int prime[MAX_PRIME + 1];

void make_prime(void)
{
    int i;
    int j;
    int n;
    int c;
    
    for (i = 2; i * i <= MAX_PRIME; i++){
        if (prime[i] == 0){
            for (j = i * 2; j <= MAX_PRIME; j += i){
				prime[j] = 1;
            }
        }
    }
}
	

int main(void)
{
	int n;
	int count;
	int s;
	int i, j;

	make_prime();
	
	while (1){
		scanf("%d", &n);
		if (n == 0){
			break;
		}

		count = 0;
		for (i = 2; i <= n; i++){
			if (prime[i] == 0){
				s = 0;
				for (j = i; j<= n; j++){
					if (prime[j] == 0){
						s += j;
//printf("[%d %d]", j, s);
						if (s == n){
							count++;
//printf("<-OK ");
						}
						else if (s > n){
//printf("<-NG ");
							break;
						}
					}
				}
			}
		}
		
		printf("%d\n", count);
	}
	return (0);
}