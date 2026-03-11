#include<stdio.h>

int nn[51]={0},ans=0;

int solve(int keta,int han,int tuduki){
//	puts("OK");
	if(keta==-1)return 1;
	if(han==1){
		int tmp=tuduki+9*(keta+1);
		if(tmp>ans)ans=tmp;
		return 0;
	}
	else{
		solve(keta-1,0,tuduki+nn[keta]);
		if(nn[keta]!=0){
			solve(keta-1,1,tuduki+nn[keta]-1);
		}
		return 0;
	}
}

int main(){
	long long n;
	scanf("%lld",&n);
	int keta=0,i;
	while(n>0){
		nn[keta]=n%10;
		n=(n-nn[keta])/10;
		keta++;
	}
	//for(i=0;i<50;i++)printf("%d,",nn[i]);
	solve(50,0,0);
	printf("%d\n",ans);
	return 0;
}