#include <stdio.h>

int main(void){
	int p,c, j;
	int n,r;
	int dx[50], a[50], b[50];
	int i, tempa, tempb, cnt;
	
	while(1){
		
		scanf("%d %d", &n, &r);
		if (n==0 && r==0) break;
		for(j = 0; j < n; j++){
				dx[j] = (j + 1);
		}
		for(i=0; i<r; i++){
			scanf("%d %d", &p, &c);
			tempa = 0;
			for(j = n - p; j != n - p - c; j--){
				a[tempa] = dx[j];
				tempa++;
			}
			tempb = 0;
			cnt = 0;
			for(j = n - 1; j > n - p; j--){
				b[tempb] = dx[j];
				tempb++;
				cnt++;
			}
			tempa = 0;
			tempb = 0;
			for(j = n - 1; j > n - 1 - c; j--){
				dx[j] = a[tempa];
				tempa++;
			}
			for(; j > n - 1 - c - cnt; j--){
				dx[j] = b[tempb];
				tempb++;
			}
		}		
		printf("%d\n", dx[n - 1]);
	}
	
	return(0);
	
}