#include <stdio.h>
#include <string.h>

int main(void)
{
	static char prime[1000001];
    static char ok[1000001];
	int i, j;
	int n, t;
    int a[30];
    int max;
	
	for (i = 3; i <= 1000000; i += 2){
		prime[i] = 1;
	}
	prime[2] = 1;
	
	for (i = 3; i <= 1000; i += 2){
		if (prime[i] == 1){
			for (j = 3; j <= 1000000 / i; j += 2){
				prime[i * j] = 0;
			}
		}
	}
	
	while (1){
        scanf("%d%d", &t, &n);
		
        if (t + n == 0){
            break;
        }
        
        for (i = 0; i < t; i++){
            scanf("%d", &a[i]);
        }
        
        memset(ok, 0, sizeof(ok));
        ok[0] = 1;
        max = -100;
        for (i = 0; i < t; i++){
            for (j = 0; j + a[i] <= n; j++){
                if (ok[j] != 0){
                    ok[j + a[i]] = ok[j];
                    if (j + a[i] > max && prime[j + a[i]]){
                        max = j + a[i];
                    }
                }
            }
        }
        
        if (max != -100){
		    printf("%d\n", max);
        }
        else {
            printf("NA\n");
        }
	}
	
	return (0);
}