#include <stdio.h>
#include <stdlib.h>

int cmp(const void* a,const void* b){
	return *(int*)a - *(int*)b;
}
int b[200000]={};
int main(void){
	int n,k;
	scanf("%d%d",&n,&k);
	int i,c=0;
	int a[n+1];
	for(i=0;i<n;i++)scanf("%d",&a[i]);
	qsort(a,n,sizeof(int),cmp);
	for(i=0;i<n;i++){
		b[c]++;
		while(a[i]==a[i+1]){
			i++;
			b[c]++;
			if(i>=n)continue;
		}
		c++;
	}
	//cはボールの種類数で,b[i]にはその数が入る
	//ボールに書かれている整数がK種類以下にする最適な方法は、
	//b[i]が小さいヤツから消していく
	qsort(b,c,sizeof(int),cmp);
	int ans = 0;
	for(i=0;i<c-k;i++){
		ans += b[i];
	}
	printf("%d",ans);
	return 0;
}