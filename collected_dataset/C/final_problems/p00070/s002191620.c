// AOJ Volume 0 Problem 0070

#include <stdio.h>
#include <string.h>

int no;
int n, s;

void combination(int num[], int depth, int sum)
{
	int i;
	
	if (s < sum){
		return;
	}

	if (depth == 1){
		if (s - sum <= 9 && num[s - sum] == 0){
			no++;
		}
		return;
	}
	
	
	for (i = 9; i >= 0; i--){
		if (num[i] == 0){
			num[i] = 1;
			combination(num, depth - 1, sum + i * depth);
			num[i] = 0;
			
		}
	}
}

int main(void)
{
	int num[10];
	
	while (scanf("%d%d", &n, &s) != EOF){
		no = 0;
		memset(num, 0, sizeof(num));
		combination(num, n, 0);
		
		printf("%d\n", no);
	}
	
	return (0);
}