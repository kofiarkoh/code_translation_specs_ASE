#include<stdio.h>

int main(void)
{
	int n,m,k;
	int s=0;
	int t=0;
	int temp;
	int i;
	
	scanf("%d %d %d", &n, &m, &k);
	for(i=0;i<n;i++){
		scanf("%1d", &temp);
		s <<= 1;
		s |= temp;
	}
	for(i=0;i<m;i++){
		scanf("%1d", &temp);
		t <<= 1;
		t |= temp;
	}
	for(i=0;i<k;i++){
		n=(s&t);
		s+=n;
		t+=n;
	}
	int j;
	int li[10000]={0};
	for(i=0;s>0;i++){
		li[i]=s%2;
		s/=2;
	}
	for(j=i-1;j>=0;j--){
		printf("%d", li[j]);
	}
	for(i=0;t>0;i++){
		li[i]=t%2;
		t/=2;
	}
	putchar('\n');
	for(j=i-1;j>=0;j--){
		printf("%d", li[j]);
	}
	return 0;
}