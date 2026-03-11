#include<stdio.h>

int main(){
	int n, a[100], i, target, mid, max, min, p, t = 0;
	while (1){
		scanf("%d",&n);
		if (n == 0) break;
		p = n;
		max = n - 1;
		min = 0;
		for (i = 0; i < n; i++){
			scanf("%d",&a[i]);
		}
		scanf("%d",&target);
		mid = (min + max) / 2;

		while (1){
			t++;
			if (a[mid] == target){
				break;
			}
			if (a[mid] < target){
				min = mid + 1;
				mid = (min + max) / 2;
			}
			else if (a[mid] > target){
				max = mid - 1;
				mid = (min + max) / 2;
			}
			if (min > max)break;
		}
		printf("%d\n",t);
		t = 0;
	}

	return 0;
}