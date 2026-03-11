#include <stdio.h>

int main(){
	int i,n,m;
	int num[8];
	scanf("%d",&n);
	for(i=0;i<n;i++){
		int max=0;
		int min=0;
		scanf("%d",&m);
		int j,k;
		for(j=0;j<8;j++){
			num[j]=m%10;
			m/=10;
		}
		for(j=0;j<8;j++){
		for(k=j;k<8;k++){
			int t;
			if(num[j]>num[k]){
				t=num[j];
				num[j]=num[k];
				num[k]=t;
			}
		}
		}
		
		for(j=0;j<8;j++){
			min+=num[j];
			if(j<7)min*=10;
		}
		for(j=7;j>=0;j--){
			max+=num[j];
			if(j>0)max*=10;
		}
		printf("%d\n",max-min);
	}
	return 0;
}