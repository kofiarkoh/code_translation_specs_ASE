#include<stdio.h>

#define N 100000

int abs(int a);

int main(void){
	int i;
	int j;
	int n;
	int k;
	int h[N];
	int min[N];
	scanf("%d %d",&n,&k);
	for(i=0;i<n;i++){
		scanf("%d",&h[i]);
	}
	min[0]=0;
	for(i=1;i<n;i++){
		min[i]=min[i-1]+abs(h[i]-h[i-1]);
		if(i<k){
			for(j=1;j<=i;j++){
				if(min[i]>(min[i-j] + abs(h[i]-h[i-j]))){
					min[i]=min[i-j] + abs(h[i]-h[i-j]);
				}
			}
		}else{
			for(j=1;j<=k;j++){
				if(min[i]>(min[i-j] + abs(h[i]-h[i-j]))){
					min[i]=min[i-j] + abs(h[i]-h[i-j]);
				}
			}
		}
	}
	printf("%d",min[n-1]);
	return 0;
}

int abs(int a){
	int ret;
	if(a<0){
		ret=(-1)*a;
	}else{
		ret=a;
	}
	return ret;
}