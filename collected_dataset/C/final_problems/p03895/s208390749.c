#include<stdio.h>
int main(){
	int n,s=0,a,b,i,M=86400,d[100010]={};
	scanf("%d",&n);
	for(i=0;i<n;i++){
		scanf("%d %d",&a,&b);
		s=(s+a)%M;
		d[s]++;
		s=(s+b)%M;
	}
	for(i=s=0;i<10801;i++)s+=d[i];
	a=s;
	for(i=0;i<M;i++){
		s+=d[(i+10801)%M]-d[i];
		if(a<s)a=s;
	}
	printf("%d\n",a);
	return 0;
}