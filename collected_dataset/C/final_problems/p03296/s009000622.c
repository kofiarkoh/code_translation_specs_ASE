#include <stdio.h>
int main()
{
	int n,a[110];
	scanf("%d",&n);
	for(int i=0;i<=n-1;i++){
		scanf("%d",&a[i]);
	}
	int magic=0;
	for(int j=1;j<=n-2;j++){
		if(a[j-1]==a[j] && a[j]==a[j+1]){
			magic++;
			a[j]++;
		}
	}
	for(int j=1;j<=n-2;j++){
		if(a[j-1]==a[j] || a[j]==a[j+1]){
			magic++;
			a[j]=a[j-1]+a[j+1];
		}
	}
	printf("%d\n",magic);
	return 0;
}