#include <stdio.h>
int main(void) {
	int n, x, ans;
	while(1){
		ans = 0;

		scanf("%d %d", &n, &x);
		if(n == 0 && x == 0) break;

		if(n < 3 || x < 6){
			ans = 0;
		}else if(x > n + (n - 1) + (n - 2)){
			ans = 0;
		}else{
			for(int i = 1;i <= n - 2;i++){
			for(int j = i + 1;j <= n - 1;j++){
				if(i == j) continue;
				for(int k =  j + 1;k <= n;k++){
					if(j == k || i == k) continue;
					if((i + j + k) == x) ans++;
				}
			}
		}
	}
	printf("%d\n", ans);
}
return 0;
}
