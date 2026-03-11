#include <stdio.h>
#include <math.h>

int fact(int n);

int main()
{
	int i, j, n, k, cnt, ans = 0; 
	
	scanf("%d %d", &n, &k);		
	
	if(k % 2 == 0){
		cnt = 0;
		for(i = k / 2; i <= n; i += k){
			cnt++;
		}
		if(cnt >= 1){
			ans += (int)pow((double)cnt, (double)3);
		}
	}
	
	cnt = n / k;
	if(cnt >= 1){
		ans += (int)pow((double)cnt, (double)3);
	}
	
	printf("%d", ans);


	return 0;
}