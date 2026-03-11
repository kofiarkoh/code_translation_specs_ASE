#include<stdio.h>
#include<stdlib.h>

int compare_int(const void *a,const void *b){
	return *(int*)a - *(int*)b;
}

int main(void){
	int n;
	int i,j;
	int x[1000000];
	int count = 0;
	int ans = -1;
	while(scanf("%d",&n)){
		if(n == 0){
			break;
		}
		for(i = 0;i < n;i++){
			scanf("%d",&x[i]);
		}
		qsort(x,n,sizeof(int),compare_int);
		for(i = 0;i < n;i++){
			if(x[i] == x[i+1]){
				for(j = i;;j++){
					if(x[j] == x[j+1]){
						count++;
					}
					else{
						break;
					}
				}
			}
			if(count > n / 2){
				ans = x[i];
				break;
			}
			count = 0;
		}
		if(ans == -1){
			printf("NO COLOR\n");
			count = 0;
		}
		else{
			printf("%d\n",ans);
			ans = -1;
			count = 0;
		}
	}
	return 0;
}