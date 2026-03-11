#include <stdio.h>
#include <string.h>

#define MAX_PRIME	(1000000)
#define IS_PRIME(n)	(prime[j]==0?1:0)

int prime[MAX_PRIME + 1];
int fare[MAX_PRIME + 1];
int fare_next[MAX_PRIME + 1];


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
	int n, x;
	int price[30];
	int i, j, k;
	
	make_prime();
	
	while (1){
		scanf("%d%d", &n, &x);
		if (n == 0 && x == 0){
			break;
		}
		
		for (i = 0; i < n; i++){
			scanf("%d", &price[i]);
		}
		
		memset(fare_next, 0, sizeof(fare_next));
		fare_next[0] = 1;
		for (i = 0; i < n; i++){

			memcpy(fare, fare_next, sizeof(fare_next));
			for (j = price[i]; j <= x; j += price[i]){
				for (k = 0; k <= x; k++){
					if (fare[k] != 0){
						if (k + j <= x){
							fare_next[k + j] = 1;
						}
					}
				}
			}
		}
		memcpy(fare, fare_next, sizeof(fare_next));
		
//for (i = 1; i <= x; i++){
//	printf("[%d=%d]", i, fare[i]);
//}

		for (i = x; i >= 1; i--){
			if (fare[i] != 0 && prime[i] == 0){
				printf("%d\n", i);
				break;
			}
		}
		if (i == 0){
			printf("NA\n");
		}
	}
	return (0);
}