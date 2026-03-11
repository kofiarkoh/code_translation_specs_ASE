#include <stdio.h>
int main(void)
{
	int n, m, s[5], c[5], i, ans[4], con = 0, l;
	for(i = 0; i < 4; i++){
		ans[i] = 0;
	}
	
	scanf("%d %d", &n, &m);
	for(i = 0; i < m; i++){
		scanf("%d %d", &s[i], &c[i]);
	}
	
	for(i = 0; i < m; i++){
		if(con == 1){
			break;
		}
		if(s[i] == 1 && c[i] == 0 && n != 1){
			printf("-1");
			con = 1;
			break;
		}
		for(l = 0; l < m; l++){
			if(i != l && s[i] == s[l] && c[i] != c[l]){
				printf("-1");
				con = 1;
				break;
			}
		}
	}
	
	if(con == 0){
		for(i = 0; i < m; i++){
			ans[s[i]] = c[i];
		}
		if(n != 1 && ans[1] == 0){
			ans[1] = 1;
		}
		for(i = 1; i <= n; i++){
			printf("%d", ans[i]);
		}
	}
	
	return 0;
}