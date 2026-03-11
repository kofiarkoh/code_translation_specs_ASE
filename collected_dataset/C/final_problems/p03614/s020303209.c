#include<stdio.h>
#define M 100000+5
int main(){
	int num=0,n,a[M];
	scanf("%d",&n);
	for(int i=1;i<=n;i++){
		scanf("%d",&a[i]);
		if(i>1 && a[i-1]==i-1){
			int t=a[i-1];
			a[i-1]=a[i];
			a[i]=t;
			num++;
		}
	}
	int num1=1;
	for(;;){
		int f=1;
		for(int i=num1;i<=n;i++){
			if(a[i]==i && i<n){
				int t=a[i+1];
				a[i+1]=a[i];
				a[i]=t;
				num1=i+1;
				f=0;
				num++;
				break;
			}
		}
		if(f==1)break;
	}
	printf("%d",num);
} 