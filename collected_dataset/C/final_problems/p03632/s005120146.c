#include<stdio.h>
int main(){
	int A[110],a,b,c,d,i,sum=0;
	for(i=0;i<100;i++) A[i]=0;
	scanf("%d%d%d%d",&a,&b,&c,&d);
	for(i=a;i<b;i++) A[i]++;
	for(i=c;i<d;i++) A[i]++;
	for(i=0;i<100;i++){
		if(A[i]==2) sum++;
	}
	printf("%d\n",sum);
	return 0;
}