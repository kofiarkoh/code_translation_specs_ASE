#include<stdio.h>
int main(void)
{
	int n;
	int cnt[7];
	int i,j;
	double sin;
	
	scanf("%d",&n);
	for(i=1;i<=6;i++){
		cnt[i]=0;
	}
	for(i=1;i<=n;i++){
		scanf("%lf",&sin);
		if(sin<165.0){
			cnt[1]++;
		}
		else if(sin<170.0){
			cnt[2]++;
		}
		else if(sin<175.0){
			cnt[3]++;
		}	
		else if(sin<180.0){
			cnt[4]++;
		}
		else if(sin<185.0){
			cnt[5]++;
		}
		else if(sin>=185.0){
			cnt[6]++;
		}
	}
	for(i=1;i<=6;i++){
		printf("%d:",i);
		for(j=1;j<=cnt[i];j++){
			printf("*");
		}
		printf("\n");
	}
	return 0;
}