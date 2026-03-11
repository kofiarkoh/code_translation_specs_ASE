#include <stdio.h>
int main(void){
	int N,M;
	scanf("%d %d",&N,&M);
	int a[N][N];
	long i,j;
	int ans=0;
	for(i=0;i<N;i++){
		scanf("%d",&a[i][0]);
		a[i][0]%=M;
	}
	for(i=0;i<N;i++){
		for(j=0;j<N;j++){
			a[i][j]=a[i][0];
		}
	}
	/*for(i=0;i<N;i++){
		for(j=0;j<N;j++){
			printf("a[%d][%d]=%d\n",i,j,a[i][j]);
		}
	}*/
	for(i=0;i<N;i++){
		if(a[i][i]%M==0){
			ans++;
		}
		for(j=1;j+i<N;j++){
			a[i+j][i]+=a[i+j-1][i];
			a[i+j][i]%=M;
			if(a[i+j][i]==0){
				ans++;
			}
		}
	}
	printf("%d",ans);
	return 0;
}